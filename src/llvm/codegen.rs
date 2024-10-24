use std::cell::RefCell;
use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::values::PointerValue;
use inkwell::OptimizationLevel;

use crate::lexer::types::DATATYPE;
use crate::llvm::builder;
use crate::parser::nodes::{
    AssignmentParserNode, ConditionalIfParserNode, FunctionParserNode, LoopParserNode, ParserType,
    ReturnNode, VariableCallParserNode,
};
use crate::parser::types::ParserTypes;

type MainFunc = unsafe extern "C" fn() -> u32;

pub struct FunctionStore<'ctx> {
    pub name: String,
    pub args: HashMap<String, VariableStore<'ctx>>,
}

impl FunctionStore<'_> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            args: HashMap::new(),
        }
    }
}

pub struct VariableStore<'ctx> {
    pub ptr: PointerValue<'ctx>,
    pub is_mutable: bool,
    pub datatype: DATATYPE,
}

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,
    pub execution_engine: ExecutionEngine<'ctx>,
    pub tokens: Vec<Box<dyn ParserType>>,
    pub variables: RefCell<Vec<FunctionStore<'ctx>>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, tokens: Vec<Box<dyn ParserType>>) -> Self {
        let module = context.create_module("main");
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .expect("failed to create execution engine");
        Self {
            context: &context,
            module,
            builder: context.create_builder(),
            execution_engine,
            tokens,
            variables: RefCell::new(Vec::new()),
        }
    }

    pub fn compile(&self, build: bool) -> Option<u32> {
        for node in &self.tokens {
            // functions should be the only type of node at the top level
            match node.get_type() {
                ParserTypes::FUNCTION => {
                    let downcast_node = node.any().downcast_ref::<FunctionParserNode>().unwrap();

                    self.add_function(downcast_node);
                }
                _ => todo!(),
            }
        }
        if build {
            builder::build_ir(&self.module);
            None
        } else {
            unsafe {
                let exec: JitFunction<MainFunc> =
                    self.execution_engine.get_function("main").unwrap();
                Some(exec.call())
            }
        }
    }

    pub fn nested_codegen(
        &self,
        body: &Vec<Box<dyn ParserType>>,
        func_name: &str,
        ret_type: &DATATYPE,
    ) {
        for node in body {
            match node.get_type() {
                ParserTypes::VARIABLE => {
                    let downcast_node = node.any().downcast_ref::<AssignmentParserNode>().unwrap();
                    self.add_variable(func_name, downcast_node);
                }
                ParserTypes::VARIABLE_CALL => {
                    let downcast_node =
                        node.any().downcast_ref::<VariableCallParserNode>().unwrap();
                    self.mod_variable(func_name, downcast_node);
                }
                ParserTypes::CONDITIONAL => {
                    let downcast_if = node
                        .any()
                        .downcast_ref::<ConditionalIfParserNode>()
                        .unwrap();
                    self.add_conditional_if(func_name, downcast_if);
                }
                ParserTypes::LOOP => {
                    let downcast_node = node.any().downcast_ref::<LoopParserNode>().unwrap();
                    self.add_loop(func_name, downcast_node);
                }
                ParserTypes::RETURN => {
                    let downcast_node = node.any().downcast_ref::<ReturnNode>().unwrap();
                    self.add_return(downcast_node, func_name, ret_type);
                }
                _ => todo!(),
            }
        }
    }
}

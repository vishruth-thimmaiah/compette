use std::cell::RefCell;
use std::collections::HashMap;

use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::values::PointerValue;

extern crate stdlib;

use crate::builder;
use lexer::types::Datatype;
use parser::{
    nodes::{
        AssignmentParserNode, ConditionalIfParserNode, ForLoopParserNode, FunctionCallParserNode,
        FunctionParserNode, ImportParserNode, LoopParserNode, ParserType, ReturnNode,
        StructDefParserNode, VariableCallParserNode,
    },
    types::ParserTypes,
};

#[derive(Debug)]
pub struct FunctionStore<'ctx> {
    pub name: String,
    pub vars: HashMap<String, VariableStore<'ctx>>,
}

impl FunctionStore<'_> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            vars: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct VariableStore<'ctx> {
    pub ptr: PointerValue<'ctx>,
    pub is_mutable: bool,
    pub datatype: Datatype,
}

pub struct StructStore {
    pub name: String,
    pub fields: Vec<String>,
}

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,
    pub execution_engine: Option<ExecutionEngine<'ctx>>,
    tokens: Vec<Box<dyn ParserType>>,
    pub variables: RefCell<Vec<FunctionStore<'ctx>>>,
    pub structs: RefCell<Vec<StructStore>>,
    pub imports: RefCell<Vec<Vec<String>>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, tokens: Vec<Box<dyn ParserType>>, use_jit: bool) -> Self {
        let module = context.create_module("main");
        let execution_engine = if use_jit {
            Some(
                module
                    .create_jit_execution_engine(OptimizationLevel::None)
                    .expect("failed to create execution engine"),
            )
        } else {
            None
        };
        Self {
            context: &context,
            module,
            builder: context.create_builder(),
            execution_engine,
            tokens,
            variables: RefCell::new(Vec::new()),
            structs: RefCell::new(Vec::new()),
            imports: RefCell::new(Vec::new()),
        }
    }

    pub fn compile(&self, build: bool, run: bool) -> Option<i32> {
        for node in &self.tokens {
            match node.get_type() {
                ParserTypes::IMPORT => {
                    let downcast_node = node.any().downcast_ref::<ImportParserNode>().unwrap();
                    self.add_import(downcast_node);
                }
                ParserTypes::FUNCTION => {
                    let downcast_node = node.any().downcast_ref::<FunctionParserNode>().unwrap();
                    self.add_function(downcast_node);
                }
                ParserTypes::STRUCT => {
                    let downcast_node = node.any().downcast_ref::<StructDefParserNode>().unwrap();
                    self.def_struct(downcast_node);
                }
                _ => todo!(),
            }
        }
        if build {
            builder::build_ir(&self.module, run);
            None
        } else {
            let function = self.module.get_function("main").unwrap();
            let result = unsafe {
                self.execution_engine
                    .as_ref()
                    .unwrap()
                    .run_function_as_main(function, &[])
            };
            return Some(result);
        }
    }

    pub fn nested_codegen(
        &self,
        body: &Vec<Box<dyn ParserType>>,
        func_name: &str,
        ret_type: &Datatype,
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
                    self.add_conditional_if(func_name, downcast_if, ret_type);
                }
                ParserTypes::LOOP => {
                    let downcast_node = node.any().downcast_ref::<LoopParserNode>().unwrap();
                    self.add_loop(func_name, downcast_node, ret_type);
                }
                ParserTypes::RETURN => {
                    let downcast_node = node.any().downcast_ref::<ReturnNode>().unwrap();
                    self.add_return(downcast_node, func_name, ret_type);
                }
                ParserTypes::BREAK => {
                    self.add_break(func_name);
                }
                ParserTypes::FOR_LOOP => {
                    let downcast_node = node.any().downcast_ref::<ForLoopParserNode>().unwrap();
                    self.add_for_loop(func_name, downcast_node, ret_type);
                }
                ParserTypes::FUNCTION_CALL => {
                    let downcast_node =
                        node.any().downcast_ref::<FunctionCallParserNode>().unwrap();
                    self.add_func_call(downcast_node, func_name);
                }
                _ => todo!(),
            }
        }
    }

    fn add_import(&self, node: &ImportParserNode) {
        let mut imports = self.imports.borrow_mut();

        imports.push(node.path.clone());
    }
}

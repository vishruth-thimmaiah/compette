use std::cell::RefCell;
use std::collections::HashMap;

use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::types::BasicType;
use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::OptimizationLevel;

use crate::lexer::types::{Types, DATATYPE, OPERATOR};
use crate::llvm::builder;
use crate::parser::nodes::{
    AssignmentParserNode, ConditionalIfParserNode, ExpressionParserNode, FunctionCallParserNode,
    FunctionParserNode, ParserType, ReturnNode, ValueParserNode,
};
use crate::parser::types::ParserTypes;

type MainFunc = unsafe extern "C" fn() -> u32;

struct FunctionStore<'ctx> {
    name: String,
    args: HashMap<String, PointerValue<'ctx>>,
}

impl FunctionStore<'_> {
    fn new(name: String) -> Self {
        Self {
            name,
            args: HashMap::new(),
        }
    }
}

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    module: Module<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
    tokens: Vec<Box<dyn ParserType>>,
    variables: RefCell<Vec<FunctionStore<'ctx>>>,
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

    pub fn jit_compile(&self, build: bool) -> Option<u32> {
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

    fn nested_codegen(
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
                ParserTypes::CONDITIONAL => {
                    let downcast_if = node
                        .any()
                        .downcast_ref::<ConditionalIfParserNode>()
                        .unwrap();
                    self.add_conditional_if(func_name, downcast_if);
                }
                ParserTypes::RETURN => {
                    let downcast_node = node.any().downcast_ref::<ReturnNode>().unwrap();
                    self.add_return(downcast_node, func_name, ret_type);
                }
                _ => todo!(),
            }
        }
    }

    fn add_variable(&self, func_name: &str, node: &AssignmentParserNode) {
        let alloc = self.new_ptr(node);
        self.variables.borrow_mut().iter_mut().for_each(|x| {
            if x.name == func_name {
                x.args.insert(node.var_name.clone(), alloc);
            }
        });

        let value = node
            .value
            .any()
            .downcast_ref::<ExpressionParserNode>()
            .unwrap();

        self.builder
            .build_store(alloc, self.add_expression(value, func_name, &node.var_type))
            .unwrap();
    }

    fn add_function(&self, node: &FunctionParserNode) {
        self.variables
            .borrow_mut()
            .push(FunctionStore::new(node.func_name.clone()));
        let args = self.def_func_args(&node.args);

        let ret_type = node.return_type.as_ref().unwrap();
        let fn_type = self.def_expr(ret_type).fn_type(&args, false);
        let function = self.module.add_function(&node.func_name, fn_type, None);

        for (index, arg) in function.get_param_iter().enumerate() {
            arg.set_name(&node.args[index].0);
        }

        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        self.nested_codegen(
            &node.body,
            &node.func_name,
            node.return_type.as_ref().unwrap(),
        );
    }

    fn add_return(&self, node: &ReturnNode, func_name: &str, ret_type: &DATATYPE) {
        let ret_expr = node
            .return_value
            .any()
            .downcast_ref::<ExpressionParserNode>()
            .unwrap();
        let ret_val = self.add_expression(ret_expr, func_name, ret_type);

        self.builder.build_return(Some(&ret_val)).unwrap();
    }

    fn add_expression(
        &self,
        node: &ExpressionParserNode,
        func_name: &str,
        req_type: &DATATYPE,
    ) -> BasicValueEnum<'ctx> {
        let left_val = match node.left.get_type() {
            ParserTypes::VALUE => {
                let value_parser_node = node.left.any().downcast_ref::<ValueParserNode>().unwrap();
                match value_parser_node.r#type {
                    Types::NUMBER => self.string_to_value(&value_parser_node.value, req_type),

                    Types::IDENTIFIER => {
                        let vars = self.variables.borrow();
                        let var_name = vars
                            .iter()
                            .filter(|x| x.name == func_name)
                            .collect::<Vec<&_>>()[0]
                            .args
                            .get(value_parser_node.value.as_str());
                        let res = {
                            if let Some(var_name) = var_name {
                                self.builder
                                    .build_load(
                                        self.def_expr(req_type),
                                        *var_name,
                                        &value_parser_node.value,
                                    )
                                    .unwrap()
                            } else if let Some(func) = self.module.get_function(func_name) {
                                func.get_params()
                                    .iter()
                                    .find(|x| {
                                        x.get_name().to_str().unwrap() == value_parser_node.value
                                    })
                                    .unwrap()
                                    .to_owned()
                            } else {
                                panic!("Invalid type");
                            }
                        };
                        res
                    }
                    _ => panic!("Invalid type"),
                }
            }
            ParserTypes::FUNCTION_CALL => {
                let downcast_node = node
                    .left
                    .any()
                    .downcast_ref::<FunctionCallParserNode>()
                    .unwrap();

                let function = self.module.get_function(&downcast_node.func_name).unwrap();
                let mut args = Vec::new();
                let params = function.get_params();
                for (index, arg) in downcast_node.args.iter().enumerate() {
                    args.push(
                        self.add_expression(arg, func_name, self.get_datatype(params[index]))
                            .into(),
                    );
                }

                self.builder
                    .build_call(function, &args, &downcast_node.func_name)
                    .unwrap()
                    .try_as_basic_value()
                    .left()
                    .unwrap()
            }
            _ => panic!("Invalid type"),
        };

        let right_val = {
            if let Some(right) = &node.right {
                let right_expr = right.any().downcast_ref::<ExpressionParserNode>().unwrap();
                self.add_expression(right_expr, func_name, req_type)
            } else {
                return left_val;
            }
        };

        match node.operator.as_ref().unwrap() {
            OPERATOR::PLUS => self.add_binary_operation(&left_val, &right_val),
            OPERATOR::MINUS => self.sub_binary_operation(&left_val, &right_val),
            OPERATOR::MULTIPLY => self.mul_binary_operation(&left_val, &right_val),
            OPERATOR::DIVIDE => self.div_binary_operation(&left_val, &right_val),
            OPERATOR::EQUAL => self.comp_binary_operation(OPERATOR::EQUAL, &left_val, &right_val),
            OPERATOR::NOT_EQUAL => {
                self.comp_binary_operation(OPERATOR::NOT_EQUAL, &left_val, &right_val)
            }
            OPERATOR::GREATER => {
                self.comp_binary_operation(OPERATOR::GREATER, &left_val, &right_val)
            }
            OPERATOR::LESSER => self.comp_binary_operation(OPERATOR::LESSER, &left_val, &right_val),
            OPERATOR::GREATER_EQUAL => {
                self.comp_binary_operation(OPERATOR::GREATER_EQUAL, &left_val, &right_val)
            }
            OPERATOR::LESSER_EQUAL => {
                self.comp_binary_operation(OPERATOR::LESSER_EQUAL, &left_val, &right_val)
            }
            _ => unreachable!(),
        }
    }

    fn add_conditional_if(&self, func_name: &str, node: &ConditionalIfParserNode) {
        let function = self.module.get_function(func_name).unwrap();
        let if_block = self.context.append_basic_block(function, "if");

        let cont = self.context.append_basic_block(function, "if_cont");

        let mut prev_block = (if_block, &node.condition);
        let mut else_if_blocks = Vec::new();

        for (index, else_if_cond) in node.else_if_body.iter().enumerate() {
            let c_name = &("cond_".to_string() + &index.to_string());
            let b_name = &("else_if_".to_string() + &index.to_string());
            let cond_eval_block = self.context.append_basic_block(function, c_name);

            let expr = self.add_expression(prev_block.1, func_name, &DATATYPE::U32);

            self.builder
                .build_conditional_branch(
                    self.to_bool(&expr).into_int_value(),
                    prev_block.0,
                    cond_eval_block,
                )
                .unwrap();

            let cond_block = self.context.append_basic_block(function, b_name);
            else_if_blocks.push(cond_block);
            self.builder.position_at_end(cond_block);
            self.nested_codegen(&else_if_cond.body, func_name, &DATATYPE::U32);

            self.add_unconditional(else_if_cond.body.last(), cont);

            self.builder.position_at_end(cond_eval_block);

            prev_block = (cond_block, &else_if_cond.condition);
        }
        let else_block = self.context.append_basic_block(function, "else");

        let expr = self.add_expression(prev_block.1, func_name, &DATATYPE::U32);

        self.builder
            .build_conditional_branch(
                self.to_bool(&expr).into_int_value(),
                prev_block.0,
                else_block,
            )
            .unwrap();

        self.builder.position_at_end(if_block);
        self.nested_codegen(&node.body, func_name, &DATATYPE::U32);

        self.add_unconditional(node.body.last(), cont);

        self.builder.position_at_end(else_block);
        self.nested_codegen(
            &node.else_body.as_ref().unwrap().body,
            func_name,
            &DATATYPE::U32,
        );

        self.add_unconditional(node.else_body.as_ref().unwrap().body.last(), cont);
        cont.move_after(else_block).unwrap();
        self.builder.position_at_end(cont);
    }

    fn add_unconditional(&self, last_item: Option<&Box<dyn ParserType>>, move_to: BasicBlock) {
        if let Some(last) = last_item {
            if last.get_type() == ParserTypes::RETURN {
                return;
            }
        }
        self.builder.build_unconditional_branch(move_to).unwrap();
    }
}

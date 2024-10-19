use std::cell::RefCell;
use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::values::{IntValue, PointerValue};
use inkwell::OptimizationLevel;

use crate::lexer::types::Types;
use crate::llvm::builder;
use crate::parser::nodes::{
    AssignmentParserNode, ExpressionParserNode, FunctionParserNode, ParserType, ReturnNode,
};
use crate::parser::types::ParserTypes;

type MainFunc = unsafe extern "C" fn() -> u32;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    module: Module<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
    tokens: Vec<Box<dyn ParserType>>,
    variables: RefCell<HashMap<String, PointerValue<'ctx>>>,
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
            variables: RefCell::new(HashMap::new()),
        }
    }

    pub fn jit_compile(&self, build: bool) -> Option<u32> {
        for node in &self.tokens {
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

    fn add_variable(&self, node: &AssignmentParserNode) {
        let alloc = self.new_ptr(node);
        self.variables
            .borrow_mut()
            .insert(node.var_name.clone(), alloc);

        let value = node
            .value
            .any()
            .downcast_ref::<ExpressionParserNode>()
            .unwrap();

        self.builder
            .build_store(alloc, self.add_expression(value, &node.var_type))
            .unwrap();
    }

    fn add_function(&self, node: &FunctionParserNode) {
        let fn_type = self.def_func(node.return_type.as_ref().unwrap());
        let function = self.module.add_function(&node.func_name, fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        let ret_node = node
            .body
            .get(1)
            .unwrap()
            .any()
            .downcast_ref::<ReturnNode>()
            .unwrap();

        let node_expr = ret_node
            .return_value
            .any()
            .downcast_ref::<ExpressionParserNode>()
            .unwrap();

        let ode = node
            .body
            .get(0)
            .unwrap()
            .any()
            .downcast_ref::<AssignmentParserNode>()
            .unwrap();
        self.add_variable(ode);

        let result = self.add_expression(node_expr, &ode.var_type);

        self.builder.build_return(Some(&result)).unwrap();
    }

    fn add_expression(&self, node: &ExpressionParserNode, req_type: &Types) -> IntValue {
        let left_val = match node.left.r#type {
            Types::NUMBER => self.def_expr(req_type)
                .const_int_from_string(&node.left.value, inkwell::types::StringRadix::Decimal)
                .unwrap(),
            Types::IDENTIFIER => self
                .builder
                .build_load(
                    self.def_expr(req_type),
                    self.variables.borrow()[node.left.value.as_str()],
                    &node.left.value,
                )
                .unwrap()
                .into_int_value(),
            _ => panic!("Invalid type"),
        };

        let right_val = {
            if let Some(right) = &node.right {
                let right_expr = right.any().downcast_ref::<ExpressionParserNode>().unwrap();
                self.add_expression(right_expr, req_type)
            } else {
                return left_val;
            }
        };

        match node.operator.as_ref().unwrap() {
            Types::PLUS => self.builder.build_int_add(left_val, right_val, "main").unwrap(),
            Types::MINUS => self.builder.build_int_sub(left_val, right_val, "main").unwrap(),
            Types::MULTIPLY => self.builder.build_int_mul(left_val, right_val, "main").unwrap(),
            Types::DIVIDE => self
                .builder
                .build_int_signed_div(left_val, right_val, "main")
                .unwrap(),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use inkwell::context::Context;

    use crate::{lexer::lexer::Lexer, llvm::func::CodeGen, parser::parser::Parser};

    #[test]
    fn check_main_func() {
        let contents = r#"
        func main() u32 {
            let u32 a = 6 * 3 - 1
            return a
        }
        "#;

        let lexer = Lexer::new(&contents).tokenize();

        let parser = Parser::new(lexer.clone()).parse();

        let context = Context::create();
        let codegen = CodeGen::new(&context, parser);
        let result = codegen.jit_compile(false);
        // unsafe { println!("{}", compiled_data.call()) };
        assert_eq!(12, result.unwrap());
    }
}

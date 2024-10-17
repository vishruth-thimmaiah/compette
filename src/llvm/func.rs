use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::values::IntValue;
use inkwell::OptimizationLevel;

use crate::lexer::types::Types;
use crate::parser::nodes::{ExpressionParserNode, FunctionParserNode, ParserType, ReturnNode};
use crate::parser::types::ParserTypes;

/// Convenience type alias for the `sum` function.
///
/// Calling this is innately `unsafe` because there's no guarantee it doesn't
/// do `unsafe` operations internally.
type MainFunc = unsafe extern "C" fn() -> u64;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
    tokens: Vec<Box<dyn ParserType>>,
}

impl<'ctx> CodeGen<'ctx> {
    fn new(context: &'ctx Context, tokens: Vec<Box<dyn ParserType>>) -> Self {
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
        }
    }

    fn jit_compile(&self) -> Option<JitFunction<MainFunc>> {
        for node in &self.tokens {
            match node.get_type() {
                ParserTypes::FUNCTION => {
                    let downcast_node = node.any().downcast_ref::<FunctionParserNode>().unwrap();

                    self.add_function(downcast_node);
                }
                _ => todo!(),
            }
        }
        unsafe { self.execution_engine.get_function("main").ok() }
    }

    fn add_function(&self, node: &FunctionParserNode) {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[], false);
        let function = self.module.add_function(&node.func_name, fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        let ret_node = node
            .body
            .get(0)
            .unwrap()
            .any()
            .downcast_ref::<ReturnNode>()
            .unwrap();

        let node = ret_node
            .return_value
            .any()
            .downcast_ref::<ExpressionParserNode>()
            .unwrap();
        let result = self.add_expression(node);

        self.builder.build_return(Some(&result)).unwrap();
    }

    fn add_expression(&self, node: &ExpressionParserNode) -> IntValue {
        let left_val = self
            .context
            .i64_type()
            .const_int_from_string(&node.left.value, inkwell::types::StringRadix::Decimal)
            .unwrap();

        let right = {
            if let Some(right) = &node.right {
                let right_expr = right.any().downcast_ref::<ExpressionParserNode>().unwrap();
                self.add_expression(right_expr)
            } else {
                return left_val;
            }
        };

        match node.operator.as_ref().unwrap() {
            Types::PLUS => self.builder.build_int_add(left_val, right, "main").unwrap(),
            Types::MINUS => self.builder.build_int_sub(left_val, right, "main").unwrap(),
            Types::MULTIPLY => self.builder.build_int_mul(left_val, right, "main").unwrap(),
            Types::DIVIDE => self
                .builder
                .build_int_signed_div(left_val, right, "main")
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
        func main() {
            return 6 * 3 - 1
        }
        "#;

        let lexer = Lexer::new(&contents).tokenize();

        let parser = Parser::new(lexer.clone()).parse();

        let context = Context::create();
        let codegen = CodeGen::new(&context, parser);
        let compiled_data = codegen.jit_compile().unwrap();
        // unsafe { println!("{}", compiled_data.call()) };
        unsafe { assert_eq!(12, compiled_data.call()) };
    }
}

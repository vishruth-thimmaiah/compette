use std::{error::Error, fmt::Display};

use inkwell::{
    OptimizationLevel,
    builder::{Builder, BuilderError},
    context::Context,
    execution_engine::ExecutionEngine,
    module::Module,
};
use new_parser::nodes::ASTNodes;
use stmt::Variables;
use structs::StructDefs;

mod block;
mod cond;
mod expr;
mod func;
mod loops;
mod ops;
mod stmt;
mod structs;
mod utils;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,
    pub execution_engine: Option<ExecutionEngine<'ctx>>,
    pub tokens: Vec<ASTNodes>,

    pub struct_defs: StructDefs<'ctx>,
    pub var_ptrs: Variables<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, tokens: Vec<ASTNodes>, with_jit: bool) -> Self {
        let builder = context.create_builder();
        let module = context.create_module("main");
        let execution_engine = with_jit.then(|| {
            module
                .create_jit_execution_engine(OptimizationLevel::None)
                .unwrap()
        });
        Self {
            context,
            builder,
            module,
            execution_engine,
            tokens,

            struct_defs: StructDefs::default(),
            var_ptrs: Variables::default(),
        }
    }

    pub fn codegen(&self) -> Result<(), CodeGenError> {
        for node in self.tokens.iter() {
            match node {
                ASTNodes::Function(func) => {
                    self.impl_function_def(func)?;
                }
                ASTNodes::StructDef(st) => {
                    self.def_struct(st)?;
                }
                ASTNodes::ImportDef(_) => todo!(),
                _ => unreachable!(),
            };
        }
        Ok(())
    }

    pub fn ir_as_string(&self) -> String {
        self.module.print_to_string().to_string()
    }

    pub fn run_with_jit(&self) -> Option<i32> {
        let function = self.module.get_function("main").unwrap();
        let exec_engine = self.execution_engine.as_ref().unwrap();
        let result = unsafe { exec_engine.run_function_as_main(function, &[]) };
        return Some(result);
    }
}

#[derive(Debug)]
pub struct CodeGenError {
    msg: String,
}
impl Display for CodeGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CodeGenError: {}", self.msg)
    }
}
impl Error for CodeGenError {}

impl CodeGenError {
    fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }

    fn from_llvm_err(err: BuilderError) -> Self {
        Self {
            msg: err.to_string(),
        }
    }
}

#[cfg(test)]
pub(crate) fn get_codegen_for_string(code: &str) -> Result<String, CodeGenError> {
    use lexer::lexer::Lexer;
    use new_parser::Parser;

    let context = Context::create();
    let lexer = Lexer::new(code).tokenize();
    let parser = Parser::new(lexer).parse();
    if let Err(err) = parser {
        return Err(CodeGenError::new(&format!("Failed to parse: {}", err)));
    }
    let parser = parser.unwrap();
    let codegen = CodeGen::new(&context, parser, false);
    codegen.codegen()?;
    Ok(codegen.ir_as_string())
}

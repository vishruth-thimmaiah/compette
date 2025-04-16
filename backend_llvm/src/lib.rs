use inkwell::{
    builder::Builder, context::Context, execution_engine::ExecutionEngine, module::Module,
};
use new_parser::nodes::ASTNodes;
use structs::StructDefs;

mod block;
mod expr;
mod func;
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
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, tokens: Vec<ASTNodes>) -> Self {
        let builder = context.create_builder();
        let module = context.create_module("main");
        let execution_engine = None;
        Self {
            context,
            builder,
            module,
            execution_engine,
            tokens,

            struct_defs: StructDefs::default(),
        }
    }

    pub fn codegen(&self) -> Result<(), ()> {
        for node in self.tokens.iter() {
            match node {
                ASTNodes::Function(func) => self.impl_function_def(func)?,
                ASTNodes::StructDef(st) => self.def_struct(st),
                ASTNodes::ImportDef(_) => todo!(),
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    pub fn string_as_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }
}

#[cfg(test)]
pub(crate) fn get_codegen_for_string(code: &str) -> Result<String, ()> {
    use lexer::lexer::Lexer;
    use new_parser::Parser;

    let context = Context::create();
    let lexer = Lexer::new(code).tokenize();
    let parser = Parser::new(lexer).parse();
    if parser.is_err() {
        return Err(());
    }
    let parser = parser.unwrap();
    let codegen = CodeGen::new(&context, parser);
    codegen.codegen()?;
    Ok(codegen.string_as_ir())
}

use inkwell::context::Context;

use crate::{lexer::lexer::Lexer, parser::Parser};

use super::codegen::CodeGen;

mod conditionals;
mod general;
mod loops;

#[allow(dead_code)]
pub fn generate_result(contents: &str) -> Option<u32> {
    let lexer = Lexer::new(&contents).tokenize();

    let parser = Parser::new(lexer.clone()).parse();

    let context = Context::create();
    let codegen = CodeGen::new(&context, parser);
    codegen.compile(false, false)
}


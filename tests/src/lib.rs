use inkwell::context::Context;

use backend_llvm::{CodeGen, run_with_jit};
use lexer::lexer::Lexer;
use parser::Parser;

mod conditionals;
mod general;
mod loops;

pub fn generate_result(contents: &[&str]) -> Option<i32> {
    let context = Context::create();
    let mut cgn = Vec::new();
    {
        for content in contents {
            let lexer = Lexer::new(&content).tokenize();
            let parser = Parser::new(lexer).parse().unwrap();
            let codegen = CodeGen::new(&context, parser, "main");
            codegen.codegen().unwrap();
            cgn.push(codegen);
        }
        return run_with_jit(&cgn);
    }
}

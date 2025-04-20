use inkwell::context::Context;

use backend_llvm::CodeGen;
use lexer::lexer::Lexer;
use parser::Parser;

mod conditionals;
mod general;
mod loops;

pub fn generate_result(contents: &str) -> Option<i32> {
    let lexer = Lexer::new(&contents).tokenize();
    let parser = Parser::new(lexer).parse().unwrap();
    let context = Context::create();
    let codegen = CodeGen::new(&context, parser, true);
    codegen.codegen().unwrap();
    return codegen.run_with_jit();
}

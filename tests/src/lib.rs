use inkwell::context::Context;

use lexer::lexer::Lexer;
use llvm::codegen::CodeGen;
use parser::Parser;
use new_parser::Parser as NewParser;
use backend_llvm::CodeGen as NewCodeGen;

mod conditionals;
mod general;
mod loops;

#[allow(dead_code)]
pub fn generate_result(contents: &str) -> Option<i32> {
    let lexer = Lexer::new(&contents).tokenize();

    let parser = Parser::new(lexer.clone()).parse();

    let context = Context::create();
    let codegen = CodeGen::new(&context, parser, true);
    codegen.compile(false, false)
}

pub fn generate_new_result(contents: &str) -> Option<i32> {
    let lexer = Lexer::new(&contents).tokenize();
    let parser = NewParser::new(lexer).parse().unwrap();
    let context = Context::create();
    let codegen = NewCodeGen::new(&context, parser, true);
    codegen.codegen().unwrap();
    return codegen.run_with_jit()
}

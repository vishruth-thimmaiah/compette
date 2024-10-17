use std::{env::args, fs};

use inkwell::context::Context;
use lexer::lexer::Lexer;
use llvm::func::CodeGen;
use parser::parser::Parser;

mod lexer;
mod llvm;
mod parser;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        let contents = fs::read_to_string(&args[1]).unwrap();

        let lexer = Lexer::new(&contents).tokenize();

        let parser = Parser::new(lexer.clone()).parse();

        if args.len() == 3 {
            if args[2] == "lexer" {
                println!("{:#?}", lexer);
            } else if args[2] == "ast" {
                println!("{:#?}", parser);
            } else if args[2] == "lexer-ast" {
                println!("Lexer{:#?}\n", lexer);
                println!("ast{:#?}", parser);
            }
        }

        let context = Context::create();
        let codegen = CodeGen::new(&context, parser);
        let _ = codegen.jit_compile(true);
    } else {
        println!("Usage: sloppee <file>");
    }
}

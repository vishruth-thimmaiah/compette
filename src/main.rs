use std::{env::args, fs};

use inkwell::context::Context;
use lexer::lexer::Lexer;
use llvm::codegen::CodeGen;
use parser::parser::Parser;

mod args;
mod lexer;
mod llvm;
mod parser;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        let contents = fs::read_to_string(&args[1]).unwrap();
        let args = args::parse_args(args);

        let lexer = Lexer::new(&contents).tokenize();
        if args.print_lexer_ouput {
            println!("{:#?}", lexer);
        }

        let parser = Parser::new(lexer.clone()).parse();
        if args.print_ast_output {
            println!("{:#?}", parser);
        }

        let context = Context::create();
        let codegen = CodeGen::new(&context, parser);

        if args.use_jit {
            let output = codegen.compile(false);
            println!("Exit Code: {}", output.unwrap());
        } else {
            codegen.compile(true);
        }
    } else {
        println!("Usage: sloppee <file>");
    }
}

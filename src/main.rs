use std::{env::args, fs};

use lexer::lexer::Lexer;
use parser::parser::Parser;

mod lexer;
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
            }
            else if args[2] == "ast" {
                println!("{:#?}", parser);
            }
            else if args[2] == "lexer-ast" {
                println!("Lexer{:#?}\n", lexer);
                println!("ast{:#?}", parser);
            }
        }

    } else {
        println!("Usage: comp <file>");
    }
}

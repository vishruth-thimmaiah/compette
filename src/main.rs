use std::{env::args, fs};

use lexer::lexer::Lexer;
use parser::parser::Parser;

mod lexer;
mod parser;

fn main() {
    let file: Vec<String> = args().collect();

    if file.len() == 2 {
        let contents = fs::read_to_string(&file[1]).unwrap();

        let tokens = Lexer::new(&contents).tokenize();

        let parser = Parser::new(tokens).parse();
        println!("{:#?}", parser);
    }
}

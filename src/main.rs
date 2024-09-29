use std::{env::args, fs};

use lexer::lexer::Lexer;

mod lexer;

fn main() {
    let file: Vec<String> = args().collect();

    println!("{:?}", file);

    if file.len() == 2 {
        let contents = fs::read_to_string(&file[1]).unwrap();

        println!("{:?}", Lexer::new(&contents).tokenize());
    }
}

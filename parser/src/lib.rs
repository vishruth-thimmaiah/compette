use std::{backtrace::Backtrace, process::exit};

use nodes::ParserType;

use lexer::lexer::Token;

mod expr;
mod flow_control;
mod funcs;
mod imports;
pub mod nodes;
mod parser;
mod tests;
pub mod types;
mod values;

pub struct Parser {
    tree: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(lexer_tokens: Vec<Token>) -> Self {
        Self {
            tree: lexer_tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Box<dyn ParserType>> {
        self.parse_scope()
    }
}

pub fn parser_error(parser: &Parser, msg: &str) -> ! {
    eprintln!("ParserError");
    let current_token = parser.get_current_token();
    let final_msg = if msg.contains("invalid token") {
        format!("{} => {:?}", msg, current_token.r#type)
    } else {
        msg.to_string()
    };
    eprintln!(
        "Error at line: {}, column: {}\nError type: {}\n",
        current_token.line + 1,
        current_token.column + 1,
        final_msg
    );
    eprintln!("{}", Backtrace::capture());
    exit(1)
}

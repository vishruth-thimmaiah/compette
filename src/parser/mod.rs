use nodes::ParserType;

use crate::lexer::lexer::Token;

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

use crate::lexer::{lexer::Token, types::Types};

mod nodes;

mod test;

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

pub struct ParserError;

impl Iterator for Parser {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tokens.len() {
            // Return the token at the current index and increment the index
            let token = self.tokens[self.index].clone();
            self.index += 1;
            Some(token)
        } else {
            // No more tokens to return
            None
        }
    }
}

impl Parser {
    pub fn new(lexer_tokens: Vec<Token>) -> Self {
        Self {
            tokens: lexer_tokens,
            index: 0,
        }
    }

    pub fn peek(&self) -> Option<Token> {
        self.tokens.get(self.index).cloned()
    }

    pub fn next_with_type(&mut self, token_type: Types) -> Result<Token, ParserError> {
        let token = self.next().unwrap();
        if token.r#type != token_type {
            return Err(ParserError);
        }
        Ok(token)
    }

    pub fn peek_with_type(&self, token_type: Types) -> Result<Token, ParserError> {
        let token = self.peek().unwrap();
        if token.r#type != token_type {
            return Err(ParserError);
        }
        Ok(token)
    }
}

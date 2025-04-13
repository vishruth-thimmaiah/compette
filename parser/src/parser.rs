use lexer::{
    lexer::Token,
    types::{Delimiter, Keyword, Types},
};

use crate::parser_error;

use super::{Parser, nodes::ParserType};

impl Parser {
    pub fn get_prev_token(&self) -> Token {
        if self.position == 0 {
            return Token::default();
        }
        self.tree
            .get(self.position - 1)
            .unwrap_or(&Token::default())
            .clone()
    }

    pub fn get_next_token(&self) -> Token {
        self.tree
            .get(self.position + 1)
            .unwrap_or(&Token::default())
            .clone()
    }

    pub fn get_current_token(&self) -> Token {
        self.tree
            .get(self.position)
            .unwrap_or(&Token::default())
            .clone()
    }

    pub fn set_next_position(&mut self) {
        self.position += 1;
    }

    pub fn parse_scope(&mut self) -> Vec<Box<dyn ParserType>> {
        let mut tokens: Vec<Box<dyn ParserType>> = vec![];

        let mut nested = false;

        while self.position < self.tree.len() {
            let token_type = self.get_current_token().r#type;
            match token_type {
                Types::NL => (),
                Types::EOF => break,
                Types::KEYWORD(Keyword::STRUCT) => tokens.push(self.parse_def_struct()),
                Types::KEYWORD(Keyword::IMPORT) => tokens.push(self.parse_import()),
                Types::KEYWORD(Keyword::LET) => tokens.push(self.parse_assignment()),
                Types::KEYWORD(Keyword::IF) => tokens.push(self.parse_conditional_if()),
                Types::KEYWORD(Keyword::FUNCTION) => tokens.push(self.parse_function()),
                Types::KEYWORD(Keyword::BREAK) => tokens.push(self.parse_break()),
                Types::IDENTIFIER => tokens.push(self.parse_identifier_call()),
                Types::IDENTIFIER_FUNC => tokens.push(self.parse_function_call(None)),
                Types::IMPORT_CALL => tokens.push(self.parse_import_call()),
                Types::KEYWORD(Keyword::LOOP) => tokens.push(self.parse_loop()),
                Types::DELIMITER(Delimiter::LBRACE) => nested = true,
                // TODO: better function detecting
                Types::KEYWORD(Keyword::RETURN) => {
                    if nested {
                        tokens.push(self.parse_return())
                    }
                }
                Types::DELIMITER(Delimiter::RBRACE) => {
                    if !nested {
                        parser_error(self, "Invalid close brace");
                    }
                    break;
                }
                _ => parser_error(self, "invalid token"),
            }

            self.position += 1;
        }

        tokens
    }
}

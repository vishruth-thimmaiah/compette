use crate::{errors, lexer::types::Types};

use super::{
    nodes::{ImportParserNode, ParserType},
    Parser,
};

impl Parser {
    pub fn parse_import(&mut self) -> Box<ImportParserNode> {
        if self.get_prev_token().r#type != Types::NL {
            errors::parser_error(self, "invalid token");
        }

        let mut path = vec![];

        loop {
            let token = self.get_next_token();
            self.set_next_position();

            match token.r#type {
                Types::NL => break,
                Types::IDENTIFIER => {
                    path.push(token.value.unwrap());
                    break;
                }
                Types::IMPORT_CALL => path.push(token.value.unwrap()),
                _ => errors::parser_error(self, "invalid token"),
            }
        }
        Box::new(ImportParserNode { path })
    }

    pub fn parse_import_call(&mut self) -> Box<dyn ParserType> {
        let mut path = Vec::new();
        while self.get_current_token().r#type == Types::IMPORT_CALL {
            path.push(self.get_current_token().value.unwrap());
            self.set_next_position();
        }
        if self.get_current_token().r#type == Types::IDENTIFIER_FUNC {
            return self.parse_function_call(Some(path));
        } else {
            errors::parser_error(self, "invalid token");
        }
    }
}

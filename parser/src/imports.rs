use lexer::types::Types;

use crate::parser_error;

use super::{
    Parser,
    nodes::{ImportParserNode, ParserType},
};

impl Parser {
    pub fn parse_import(&mut self) -> Box<ImportParserNode> {
        if self.get_prev_token().r#type != Types::NL {
            parser_error(self, "invalid token");
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
                _ => parser_error(self, "invalid token"),
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
            let func_call = self.parse_function_call(Some(path));
            self.set_next_position();
            return func_call;
        } else {
            parser_error(self, "invalid token");
        }
    }
}

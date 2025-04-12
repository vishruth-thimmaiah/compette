use crate::{
    errors,
    lexer::types::{Types, Datatype, Delimiter},
};

use super::{
    nodes::{
        BreakNode, ExpressionParserNode, FunctionCallParserNode, FunctionParserNode, ReturnNode,
        ValueParserNode,
    },
    Parser,
};

impl Parser {
    pub fn parse_function(&mut self) -> Box<FunctionParserNode> {
        if self.get_prev_token().r#type != Types::NL {
            errors::parser_error(self, "invalid token")
        }

        let func_name = self.get_next_token().value.unwrap();
        self.set_next_position();

        if self.get_next_token().r#type != Types::DELIMITER(Delimiter::LPAREN) {
            errors::parser_error(self, "invalid token")
        }
        self.set_next_position();

        let mut args: Vec<(String, Datatype)> = vec![];
        loop {
            let var_name = match self.get_next_token().r#type {
                Types::DELIMITER(Delimiter::RPAREN) => break,
                Types::IDENTIFIER => self.get_next_token().value.unwrap(),
                _ => errors::parser_error(self, "invalid token"),
            };
            self.set_next_position();

            let var_type = match self.get_next_token().r#type {
                Types::DATATYPE(dt) => dt,
                _ => errors::parser_error(self, "invalid token"),
            };
            self.set_next_position();

            args.push((var_name, var_type));

            match self.get_next_token().r#type {
                Types::DELIMITER(Delimiter::RPAREN) => break,
                Types::DELIMITER(Delimiter::COMMA) => (),
                _ => errors::parser_error(self, "invalid token"),
            }
            self.set_next_position();
        }
        self.set_next_position();

        let return_type = match self.get_next_token().r#type {
            Types::DATATYPE(dt) => {
                self.set_next_position();
                dt
            }
            _ => Datatype::NONE,
        };

        if self.get_next_token().r#type != Types::DELIMITER(Delimiter::LBRACE) {
            errors::parser_error(self, "invalid token")
        }
        self.set_next_position();

        let body = self.parse_scope();
        self.set_next_position();

        return Box::new(FunctionParserNode {
            func_name,
            args,
            return_type,
            body,
        });
    }

    pub fn parse_return(&mut self) -> Box<ReturnNode> {
        if self.get_prev_token().r#type != Types::NL {
            errors::parser_error(self, "invalid token")
        }

        let condition = if self.get_next_token().r#type != Types::NL {
            self.parse_expression()
        } else {
            Box::new(ExpressionParserNode {
                left: Box::new(ValueParserNode {
                    value: "".to_string(),
                    r#type: Types::DATATYPE(Datatype::NONE),
                }),
                right: None,
                operator: None,
            })
        };
        self.set_next_position();

        Box::new(ReturnNode {
            return_value: condition,
        })
    }

    pub fn parse_break(&mut self) -> Box<BreakNode> {
        if self.get_prev_token().r#type != Types::NL {
            errors::parser_error(self, "invalid token")
        }

        Box::new(BreakNode {})
    }

    pub fn parse_function_call(
        &mut self,
        imported: Option<Vec<String>>,
    ) -> Box<FunctionCallParserNode> {
        let name = self.get_current_token().value.unwrap();

        let mut args: Vec<ExpressionParserNode> = vec![];
        self.set_next_position();
        loop {
            let token = self.get_next_token();
            if token.r#type == Types::DELIMITER(Delimiter::RPAREN) {
                break;
            } else if token.r#type == Types::DELIMITER(Delimiter::COMMA) {
                self.set_next_position();
            }
            args.push(*self.parse_expression());
        }

        return Box::new(FunctionCallParserNode {
            func_name: name,
            args,
            imported,
        });
    }
}

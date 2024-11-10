use crate::{
    errors,
    lexer::types::{Types, DATATYPE, DELIMITER, OPERATOR},
};

use super::{
    nodes::{ExpressionParserNode, ParserType, ValueIterCallParserNode, ValueParserNode},
    Parser,
};

impl Parser {
    // TODO: Add support for parenthesis
    pub fn parse_expression(&mut self) -> Box<ExpressionParserNode> {
        self.set_next_position();
        let left: Box<dyn ParserType> = match self.get_current_token().r#type {
            Types::IDENTIFIER_FUNC => self.parse_function_call(None),
            Types::IDENTIFIER => {
                if self.get_next_token().r#type == Types::DELIMITER(DELIMITER::LBRACKET) {
                    let var = self.get_current_token();
                    self.set_next_position();
                    let index = self.parse_expression();
                    if self.get_next_token().r#type != Types::DELIMITER(DELIMITER::RBRACKET) {
                        errors::parser_error(self, "Invalid array access");
                    }
                    self.set_next_position();

                    Box::new(ValueIterCallParserNode {
                        value: var.value.unwrap(),
                        index,
                    })
                } else {
                    Box::new(ValueParserNode {
                        value: self.get_current_token().value.unwrap(),
                        r#type: Types::IDENTIFIER,
                    })
                }
            }
            Types::NUMBER => Box::new(ValueParserNode {
                value: self.get_current_token().value.unwrap(),
                r#type: Types::NUMBER,
            }),
            Types::BOOL => Box::new(ValueParserNode {
                value: self.get_current_token().value.unwrap(),
                r#type: Types::BOOL,
            }),
            Types::DATATYPE(DATATYPE::STRING(str)) => Box::new(ValueParserNode {
                value: self.get_current_token().value.unwrap(),
                r#type: Types::DATATYPE(DATATYPE::STRING(str)),
            }),
            Types::DELIMITER(DELIMITER::LBRACKET) => self.parse_array(),
            _ => unreachable!(),
        };

        match self.get_next_token().r#type {
            Types::OPERATOR(operator) => match operator {
                OPERATOR::ASSIGN => Box::new(ExpressionParserNode {
                    left,
                    right: None,
                    operator: None,
                }),
                OPERATOR::PLUS
                | OPERATOR::DOT
                | OPERATOR::MINUS
                | OPERATOR::MULTIPLY
                | OPERATOR::DIVIDE
                | OPERATOR::EQUAL
                | OPERATOR::NOT_EQUAL
                | OPERATOR::GREATER
                | OPERATOR::LESSER
                | OPERATOR::GREATER_EQUAL
                | OPERATOR::LESSER_EQUAL => {
                    self.set_next_position();
                    let right = self.parse_expression();
                    return Box::new(ExpressionParserNode {
                        left,
                        right: Some(right),
                        operator: Some(operator),
                    });
                }
                OPERATOR::NOT | OPERATOR::COLON => todo!(),
            },
            Types::NL
            | Types::DELIMITER(DELIMITER::LBRACE)
            | Types::DELIMITER(DELIMITER::COMMA)
            | Types::DELIMITER(DELIMITER::RPAREN)
            | Types::DELIMITER(DELIMITER::RBRACKET) => {
                return Box::new(ExpressionParserNode {
                    left,
                    right: None,
                    operator: None,
                });
            }
            _ => errors::parser_error(self, "invalid token"),
        }
    }
}

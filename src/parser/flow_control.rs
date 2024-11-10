use crate::{
    errors,
    lexer::types::{Types, DELIMITER, KEYWORD, OPERATOR},
};

use super::{
    nodes::{
        ConditionalElseIfParserNode, ConditionalElseParserNode, ConditionalIfParserNode,
        ExpressionParserNode, ForLoopParserNode, LoopParserNode, ParserType, ValueParserNode,
    },
    Parser,
};

impl Parser {
    pub fn parse_conditional_if(&mut self) -> Box<ConditionalIfParserNode> {
        if self.get_prev_token().r#type != Types::NL {
            errors::parser_error(self, "invalid token")
        }

        let condition = self.parse_expression();
        self.set_next_position();

        let body = self.parse_scope();

        return Box::new(ConditionalIfParserNode {
            condition,
            body,
            else_if_body: self.parse_conditional_else_if(),
            else_body: self.parse_conditional_else(),
        });
    }

    fn parse_conditional_else_if(&mut self) -> Vec<ConditionalElseIfParserNode> {
        let mut else_if_body = vec![];

        loop {
            let token = self.get_next_token();
            if token.r#type != Types::KEYWORD(KEYWORD::ELSE) {
                break;
            }
            self.set_next_position();

            let token = self.get_next_token();
            if token.r#type != Types::KEYWORD(KEYWORD::IF) {
                break;
            }
            self.set_next_position();

            let condition = self.parse_expression();
            self.set_next_position();

            let body = self.parse_scope();

            else_if_body.push(ConditionalElseIfParserNode { condition, body });
        }

        else_if_body
    }

    fn parse_conditional_else(&mut self) -> Option<ConditionalElseParserNode> {
        if self.get_current_token().r#type != Types::KEYWORD(KEYWORD::ELSE) {
            return None;
        }

        if self.get_next_token().r#type != Types::DELIMITER(DELIMITER::LBRACE) {
            errors::parser_error(self, "invalid token")
        }
        self.set_next_position();

        let body = self.parse_scope();

        return Some(ConditionalElseParserNode { body });
    }

    fn parse_for_loop(&mut self) -> Box<ForLoopParserNode> {
        self.set_next_position();

        let iterator = self.parse_expression();

        self.set_next_position();
        if self.get_current_token().r#type != Types::OPERATOR(OPERATOR::ASSIGN) {
            errors::parser_error(self, "invalid token")
        }
        self.set_next_position();

        let incr_value = self.get_current_token().value.unwrap();
        self.set_next_position();

        if self.get_current_token().r#type != Types::DELIMITER(DELIMITER::COMMA) {
            errors::parser_error(self, "invalid token")
        }

        self.set_next_position();
        let index = self.get_current_token().value.unwrap();
        self.set_next_position();

        let body = self.parse_scope();
        Box::new(ForLoopParserNode {
            body,
            iterator,
            index,
            incr_value,
        })
    }

    pub fn parse_loop(&mut self) -> Box<dyn ParserType> {
        if self.get_prev_token().r#type != Types::NL {
            errors::parser_error(self, "invalid token")
        }

        if self.get_next_token().r#type == Types::KEYWORD(KEYWORD::RANGE) {
            return self.parse_for_loop();
        }

        let condition = if self.get_next_token().r#type == Types::DELIMITER(DELIMITER::LBRACE) {
            Box::new(ExpressionParserNode {
                left: Box::new(ValueParserNode {
                    value: "1".to_string(),
                    r#type: Types::BOOL,
                }),
                right: None,
                operator: None,
            })
        } else {
            self.parse_expression()
        };
        self.set_next_position();

        let body = self.parse_scope();

        return Box::new(LoopParserNode { condition, body });
    }
}

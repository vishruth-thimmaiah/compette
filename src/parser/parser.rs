use crate::{
    errors,
    lexer::{
        lexer::Token,
        types::{ArrayDetails, Types, DATATYPE, DELIMITER, KEYWORD, OPERATOR},
    },
    parser::nodes::ValueParserNode,
};

use super::nodes::{
    AssignmentParserNode, ConditionalElseIfParserNode, ConditionalElseParserNode,
    ConditionalIfParserNode, ExpressionParserNode, FunctionCallParserNode, FunctionParserNode,
    LoopParserNode, ParserType, ReturnNode, ValueIterCallParserNode, ValueIterParserNode,
    VariableCallParserNode,
};

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

    fn get_prev_token(&self) -> Token {
        if self.position == 0 {
            return Token::default();
        }
        self.tree
            .get(self.position - 1)
            .unwrap_or(&Token::default())
            .clone()
    }

    fn get_next_token(&self) -> Token {
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

    fn set_next_position(&mut self) {
        self.position += 1;
    }

    pub fn parse(&mut self) -> Vec<Box<dyn ParserType>> {
        self.parse_scope()
    }

    fn parse_scope(&mut self) -> Vec<Box<dyn ParserType>> {
        let mut tokens: Vec<Box<dyn ParserType>> = vec![];

        let mut nested = false;

        loop {
            let token_type = self.get_current_token().r#type;
            match token_type {
                Types::NL => (),
                Types::EOF => break,
                Types::KEYWORD(KEYWORD::LET) => tokens.push(self.parse_assignment()),
                Types::KEYWORD(KEYWORD::IF) => tokens.push(self.parse_conditional_if()),
                Types::KEYWORD(KEYWORD::FUNCTION) => tokens.push(self.parse_function()),
                Types::IDENTIFIER => tokens.push(self.parse_identifier_call()),
                Types::IDENTIFIER_FUNC => tokens.push(self.parse_function_call()),
                Types::KEYWORD(KEYWORD::LOOP) => tokens.push(self.parse_loop()),
                Types::DELIMITER(DELIMITER::LBRACE) => nested = true,
                // TODO: better function detecting
                Types::KEYWORD(KEYWORD::RETURN) => {
                    if nested {
                        tokens.push(self.parse_return())
                    }
                }
                Types::DELIMITER(DELIMITER::RBRACE) => {
                    if !nested {
                        errors::parser_error(self, "Invalid close brace");
                    }
                    break;
                }
                _ => errors::parser_error(self, "invalid token"),
            }

            self.position += 1;
        }

        tokens
    }

    fn parse_assignment(&mut self) -> Box<AssignmentParserNode> {
        if self.get_prev_token().r#type != Types::NL {
            errors::parser_error(self, "invalid token");
        }

        let mut var_type = match self.get_next_token().r#type {
            Types::DATATYPE(dt) => dt,
            _ => errors::parser_error(self, "invalid token"),
        };
        self.set_next_position();

        let is_array = if self.get_next_token().r#type == Types::DELIMITER(DELIMITER::LBRACKET) {
            self.set_next_position();
            if self.get_next_token().r#type != Types::DELIMITER(DELIMITER::RBRACKET) {
                errors::parser_error(self, "Invalid array declaration");
            }
            self.set_next_position();
            true
        } else {
            false
        };

        let is_mutable = match self.get_next_token().r#type {
            Types::OPERATOR(OPERATOR::NOT) => {
                self.set_next_position();
                true
            }
            _ => false,
        };

        let var_name = self.get_next_token().value.unwrap();
        self.set_next_position();

        if self.get_next_token().r#type != Types::OPERATOR(OPERATOR::ASSIGN) {
            errors::parser_error(self, "invalid token")
        }
        self.set_next_position();

        let value = self.parse_expression();

        if is_array {
            let try_downcast = value.left.any().downcast_ref::<ValueIterParserNode>();

            if try_downcast.is_none() {
                errors::parser_error(self, "Invalid array assignment");
            }

            let length = try_downcast.unwrap().value.len() as u32;

            var_type = DATATYPE::ARRAY(Box::new(ArrayDetails {
                datatype: var_type,
                length,
            }));
        }

        self.set_next_position();

        return Box::new(AssignmentParserNode {
            var_name,
            var_type,
            is_mutable,
            value,
        });
    }

    fn parse_array(&mut self) -> Box<ValueIterParserNode> {
        let mut array_contents = vec![];
        while self.get_next_token().r#type != Types::DELIMITER(DELIMITER::RBRACKET) {
            array_contents.push(*self.parse_expression());
            if self.get_next_token().r#type == Types::DELIMITER(DELIMITER::RBRACKET) {
                break;
            } else if self.get_next_token().r#type != Types::DELIMITER(DELIMITER::COMMA) {
                errors::parser_error(self, "Expected comma after array element");
            }
            self.set_next_position();
        }

        self.set_next_position();

        return Box::new(ValueIterParserNode {
            value: array_contents,
        });
    }

    // TODO: Add support for parenthesis
    fn parse_expression(&mut self) -> Box<ExpressionParserNode> {
        self.set_next_position();
        let left: Box<dyn ParserType> = match self.get_current_token().r#type {
            Types::IDENTIFIER_FUNC => self.parse_function_call(),
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
            Types::DELIMITER(DELIMITER::LBRACKET) => self.parse_array(),
            _ => unreachable!(),
        };

        match self.get_next_token().r#type {
            Types::OPERATOR(operator) => match operator {
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
                OPERATOR::NOT => todo!(),
                OPERATOR::ASSIGN => unreachable!(),
            },
            Types::NL
            | Types::DELIMITER(DELIMITER::LBRACE)
            | Types::DELIMITER(DELIMITER::COMMA)
            | Types::DELIMITER(DELIMITER::RBRACKET) => {
                return Box::new(ExpressionParserNode {
                    left,
                    right: None,
                    operator: None,
                });
            }
                _ => errors::parser_error(self, "invalid token"),
        };
    }

    fn parse_function(&mut self) -> Box<FunctionParserNode> {
        if self.get_prev_token().r#type != Types::NL {
            errors::parser_error(self, "invalid token")
        }

        let func_name = self.get_next_token().value.unwrap();
        self.set_next_position();

        if self.get_next_token().r#type != Types::DELIMITER(DELIMITER::LPAREN) {
            errors::parser_error(self, "invalid token")
        }
        self.set_next_position();

        let mut args: Vec<(String, DATATYPE)> = vec![];
        loop {
            let var_name = match self.get_next_token().r#type {
                Types::DELIMITER(DELIMITER::RPAREN) => break,
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
                Types::DELIMITER(DELIMITER::RPAREN) => break,
                Types::DELIMITER(DELIMITER::COMMA) => (),
                _ => errors::parser_error(self, "invalid token"),
            }
            self.set_next_position();
        }
        self.set_next_position();

        let return_type = match self.get_next_token().r#type {
            Types::DATATYPE(dt) => {
                self.set_next_position();
                Some(dt)
            }
            _ => None,
        };

        if self.get_next_token().r#type != Types::DELIMITER(DELIMITER::LBRACE) {
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

    fn parse_return(&mut self) -> Box<ReturnNode> {
        if self.get_prev_token().r#type != Types::NL {
            errors::parser_error(self, "invalid token")
        }

        let condition = self.parse_expression();
        self.set_next_position();

        Box::new(ReturnNode {
            return_value: condition,
        })
    }

    fn parse_identifier_call(&mut self) -> Box<VariableCallParserNode> {
        let var_name: Box<dyn ParserType> =
            if self.get_next_token().r#type == Types::DELIMITER(DELIMITER::LBRACKET) {
                let name = self.get_current_token().value.unwrap();
                self.set_next_position();
                let val = Box::new(ValueIterCallParserNode {
                    value: name,
                    index: self.parse_expression(),
                });
                self.set_next_position();
                val
            } else {
                Box::new(ValueParserNode {
                    value: self.get_current_token().value.unwrap(),
                    r#type: Types::IDENTIFIER,
                })
            };

        if self.get_next_token().r#type != Types::OPERATOR(OPERATOR::ASSIGN) {
            errors::parser_error(self, "invalid token");
        }
        self.set_next_position();
        return Box::new(VariableCallParserNode {
            var_name,
            rhs: self.parse_expression(),
        });
    }

    fn parse_function_call(&mut self) -> Box<FunctionCallParserNode> {
        let name = self.get_current_token().value.unwrap();

        // Handle function call
        let mut args: Vec<ExpressionParserNode> = vec![];
        loop {
            let token = self.get_next_token();
            if token.r#type == Types::DELIMITER(DELIMITER::RPAREN) {
                break;
            } else if token.r#type == Types::IDENTIFIER {
                args.push(ExpressionParserNode {
                    left: Box::new(ValueParserNode {
                        value: token.value.unwrap(),
                        r#type: Types::IDENTIFIER,
                    }),
                    right: None,
                    operator: None,
                });
            } else if token.r#type == Types::NUMBER {
                args.push(ExpressionParserNode {
                    left: Box::new(ValueParserNode {
                        value: token.value.unwrap(),
                        r#type: Types::NUMBER,
                    }),
                    right: None,
                    operator: None,
                });
            } else if token.r#type == Types::BOOL {
                args.push(ExpressionParserNode {
                    left: Box::new(ValueParserNode {
                        value: token.value.unwrap(),
                        r#type: Types::BOOL,
                    }),
                    right: None,
                    operator: None,
                });
            }
            self.set_next_position();
        }
        self.set_next_position();

        return Box::new(FunctionCallParserNode {
            func_name: name,
            args,
        });
    }

    fn parse_conditional_if(&mut self) -> Box<ConditionalIfParserNode> {
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

    fn parse_loop(&mut self) -> Box<LoopParserNode> {
        if self.get_prev_token().r#type != Types::NL {
            errors::parser_error(self, "invalid token")
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

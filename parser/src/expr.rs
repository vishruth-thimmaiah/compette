use lexer::{
    lexer::Token,
    types::{Datatype, Delimiter, Operator, Types},
};

use crate::parser_error;

use super::{
    Parser,
    nodes::{ExpressionParserNode, ParserType, ValueIterCallParserNode, ValueParserNode},
    types::ParserTypes,
};

impl Parser {
    pub fn parse_expression(&mut self) -> Box<ExpressionParserNode> {
        let mut operands: Vec<Box<dyn ParserType>> = Vec::new();
        let mut operators: Vec<Token> = Vec::new();

        // Values that cannot have operations performed on them.
        match self.get_next_token().r#type {
            Types::DELIMITER(Delimiter::LBRACKET) => {
                self.set_next_position();
                return Box::new(ExpressionParserNode {
                    left: self.parse_array(),
                    right: None,
                    operator: None,
                });
            }
            Types::DELIMITER(Delimiter::LBRACE) => {
                self.set_next_position();
                return Box::new(ExpressionParserNode {
                    left: self.parse_struct(),
                    right: None,
                    operator: None,
                });
            }
            Types::DATATYPE(Datatype::STRING(str)) => {
                self.set_next_position();
                return Box::new(ExpressionParserNode {
                    left: Box::new(ValueParserNode {
                        r#type: Types::DATATYPE(Datatype::STRING(str)),
                        value: self.get_current_token().value.unwrap(),
                    }),
                    right: None,
                    operator: None,
                });
            }
            _ => (),
        }

        'outer: loop {
            let token = self.get_next_token();
            match token.r#type {
                Types::NUMBER | Types::BOOL => operands.push(Box::new(ValueParserNode {
                    r#type: token.r#type,
                    value: token.value.unwrap(),
                })),
                Types::IDENTIFIER => {
                    if self.tree[self.position + 2].r#type == Types::DELIMITER(Delimiter::LBRACKET)
                    {
                        self.set_next_position();
                        self.set_next_position();

                        operands.push(Box::new(ValueIterCallParserNode {
                            value: token.value.clone().unwrap(),
                            index: self.parse_expression(),
                        }));
                    } else {
                        operands.push(Box::new(ValueParserNode {
                            r#type: token.r#type,
                            value: token.value.unwrap(),
                        }))
                    }
                }
                Types::DATATYPE(dt) => {
                    if self.get_current_token().r#type == Types::OPERATOR(Operator::CAST) {
                        operands.push(Box::new(ValueParserNode {
                            r#type: Types::DATATYPE(dt),
                            value: "".to_string(),
                        }));
                    }
                }
                Types::IMPORT_CALL => {
                    self.set_next_position();
                    operands.push(self.parse_import_call());
                    self.position -= 1;
                }
                Types::IDENTIFIER_FUNC => {
                    self.set_next_position();
                    operands.push(self.parse_function_call(None));
                }
                Types::OPERATOR(_) => {
                    while !operators.is_empty() {
                        let pop_op = &operators.last().unwrap().r#type;
                        if self.get_precedence(&token.r#type) > self.get_precedence(pop_op) {
                            break;
                        }
                        let pop = operators.pop().unwrap();
                        operands.push(Box::new(ValueParserNode {
                            r#type: pop.r#type,
                            value: "".to_string(),
                        }));
                    }
                    operators.push(token);
                }
                Types::DELIMITER(Delimiter::LPAREN) => {
                    operators.push(token);
                }
                Types::DELIMITER(Delimiter::RPAREN) => loop {
                    let pop_op = &operators.pop();
                    if let Some(op) = pop_op {
                        if op.r#type == Types::DELIMITER(Delimiter::LPAREN) {
                            break;
                        }
                        operands.push(Box::new(ValueParserNode {
                            r#type: op.r#type.clone(),
                            value: "".to_string(),
                        }));
                    } else {
                        break 'outer;
                        // errors::parser_error(self, "Parenthesis not closed.")
                    }
                },
                _ => break,
            }
            self.set_next_position();
        }
        while !operators.is_empty() {
            let value = operators.pop().unwrap();
            if value.r#type == Types::DELIMITER(Delimiter::LPAREN) {
                parser_error(self, "Parenthesis not closed.")
            }
            operands.push(Box::new(ValueParserNode {
                r#type: value.r#type,
                value: value.value.unwrap_or("".to_string()),
            }));
        }

        self.postfix_to_tree(&mut operands)
    }

    fn postfix_to_tree(
        &self,
        operands: &mut Vec<Box<dyn ParserType>>,
    ) -> Box<ExpressionParserNode> {
        let op = if operands.len() > 1 {
            let pop = operands.pop().unwrap();
            let value = pop.any().downcast_ref::<ValueParserNode>().unwrap();
            self.value_to_operator(value).unwrap()
        } else if operands.len() == 0 {
            parser_error(self, "Invalid postfix expression");
        } else {
            let token = operands.pop().unwrap();
            return Box::new(ExpressionParserNode {
                left: token,
                right: None,
                operator: None,
            });
        };

        let right: Box<dyn ParserType> = {
            let last_op = operands.last().unwrap();
            if last_op.get_type() == ParserTypes::VALUE {
                if let Types::OPERATOR(_) = last_op
                    .any()
                    .downcast_ref::<ValueParserNode>()
                    .unwrap()
                    .r#type
                {
                    self.postfix_to_tree(operands)
                } else {
                    operands.pop().unwrap()
                }
            } else {
                operands.pop().unwrap()
            }
        };

        let left: Box<dyn ParserType> = {
            let last_op = operands.last().unwrap();
            if last_op.get_type() == ParserTypes::VALUE {
                if let Types::OPERATOR(_) = last_op
                    .any()
                    .downcast_ref::<ValueParserNode>()
                    .unwrap()
                    .r#type
                {
                    self.postfix_to_tree(operands)
                } else {
                    operands.pop().unwrap()
                }
            } else {
                operands.pop().unwrap()
            }
        };

        Box::new(ExpressionParserNode {
            left,
            right: Some(right),
            operator: Some(op),
        })
    }

    fn get_precedence(&self, operator: &Types) -> usize {
        match operator {
            Types::OPERATOR(Operator::PLUS) | Types::OPERATOR(Operator::MINUS) => 1,
            Types::OPERATOR(Operator::MULTIPLY) | Types::OPERATOR(Operator::DIVIDE) => 2,
            Types::DELIMITER(Delimiter::LPAREN) => 0,
            Types::OPERATOR(Operator::CAST) => 0,
            _ => unreachable!(),
        }
    }

    fn value_to_operator(&self, value: &ValueParserNode) -> Option<Operator> {
        if let Types::OPERATOR(op) = &value.r#type {
            return Some(op.clone());
        }
        None
    }
}

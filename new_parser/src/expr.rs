use lexer::types::{Delimiter, Operator, Types};

use super::{
    Parser, Result,
    errors::ParserError,
    nodes::{ASTNodes, Expression, Literal, Variable},
};

impl Parser {
    pub(crate) fn parse_expression(&mut self) -> Result<Expression> {
        let mut operands: Vec<ASTNodes> = Vec::new();
        let mut operators: Vec<Types> = Vec::new();

        'outer: loop {
            let token = self.next().ok_or(ParserError::default())?;
            match token.r#type {
                Types::NUMBER | Types::BOOL => operands.push(ASTNodes::Literal(Literal {
                    value: token.value.unwrap(),
                    r#type: token.r#type,
                })),
                Types::IDENTIFIER => operands.push(ASTNodes::Variable(Variable {
                    name: token.value.unwrap(),
                })),
                Types::OPERATOR(ref op) => {
                    while !operators.is_empty() {
                        let pop_op = operators.last().unwrap();
                        if self.get_precedence(&token.r#type) > self.get_precedence(pop_op) {
                            break;
                        }
                        let pop = operators.pop().unwrap();
                        operands.push(ASTNodes::Token(pop));
                    }
                    operators.push(Types::OPERATOR(op.clone()));
                }
                Types::DELIMITER(Delimiter::LPAREN) => {
                    operators.push(token.r#type);
                }
                Types::DELIMITER(Delimiter::RPAREN) => loop {
                    let pop_op = &operators.pop();
                    if let Some(op) = pop_op {
                        if op == &Types::DELIMITER(Delimiter::LPAREN) {
                            break;
                        }
                        operands.push(ASTNodes::Token(op.clone()));
                    } else {
                        break 'outer;
                    }
                },
                _ => break,
            }
        }
        while !operators.is_empty() {
            let value = operators.pop().unwrap();
            if value == Types::DELIMITER(Delimiter::LPAREN) {}
            operands.push(ASTNodes::Token(value));
        }

        self.postfix_to_tree(&mut operands)
    }

    fn postfix_to_tree(&self, operands: &mut Vec<ASTNodes>) -> Result<Expression> {
        let op = if operands.len() > 1 {
            let value = operands.pop().unwrap();
            self.value_to_operator(value).unwrap()
        } else if operands.len() == 0 {
            todo!()
            // errors::parser_error(self, "Invalid postfix expression");
        } else {
            let token = operands.pop().unwrap();
            return Ok(Expression {
                left: Box::new(token),
                right: None,
                operator: None,
            });
        };

        let right = {
            let last_op = operands.last().unwrap();
            if let ASTNodes::Token(Types::OPERATOR(_)) = last_op {
                ASTNodes::Expression(self.postfix_to_tree(operands)?)
            } else {
                operands.pop().unwrap()
            }
        };

        let left = {
            let last_op = operands.last().unwrap();
            if let ASTNodes::Token(Types::OPERATOR(_)) = last_op {
                ASTNodes::Expression(self.postfix_to_tree(operands)?)
            } else {
                operands.pop().unwrap()
            }
        };

        Ok(Expression {
            left: Box::new(left),
            right: Some(Box::new(right)),
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

    fn value_to_operator(&self, value: ASTNodes) -> Option<Operator> {
        if let ASTNodes::Token(Types::OPERATOR(op)) = &value {
            return Some(op.clone());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use lexer::lexer::Lexer;
    use super::*;

    #[test]
    fn test_parse_expression() {
        let mut lexer = Lexer::new("1 + 2 * 3 - 4 / 5 ");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse_expression().unwrap();
        println!("{:#?}", ast);
        assert_eq!(
            ast,
            Expression {
                left: Box::new(ASTNodes::Expression(Expression {
                    left: Box::new(ASTNodes::Literal(Literal {
                        value: "1".to_string(),
                        r#type: Types::NUMBER
                    })),
                    right: Some(Box::new(ASTNodes::Expression(Expression {
                        left: Box::new(ASTNodes::Literal(Literal {
                            value: "2".to_string(),
                            r#type: Types::NUMBER
                        })),
                        right: Some(Box::new(ASTNodes::Literal(Literal {
                            value: "3".to_string(),
                            r#type: Types::NUMBER
                        }))),
                        operator: Some(Operator::MULTIPLY)
                    }))),
                    operator: Some(Operator::PLUS)
                })),
                right: Some(Box::new(ASTNodes::Expression(Expression {
                    left: Box::new(ASTNodes::Literal(Literal {
                        value: "4".to_string(),
                        r#type: Types::NUMBER
                    })),
                    right: Some(Box::new(ASTNodes::Literal(Literal {
                        value: "5".to_string(),
                        r#type: Types::NUMBER
                    }))),
                    operator: Some(Operator::DIVIDE)
                }))),
                operator: Some(Operator::MINUS)
            }
        );
    }
}

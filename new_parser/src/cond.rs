use lexer::types::{Delimiter, Keyword, Types};

use crate::{
    Parser, Result,
    nodes::{Block, Conditional, Expression},
};

impl Parser {
    pub(crate) fn parse_if(&mut self) -> Result<Conditional> {
        let condition = self.parse_expression(vec![Types::DELIMITER(Delimiter::LBRACE)])?;
        let block = self.parse_scoped_block()?;

        let (else_if_condition, else_if_body) = self.parse_else_if()?;

        if self
            .current_with_type(Types::KEYWORD(Keyword::ELSE))
            .is_ok()
        {
            self.prev();
        }

        Ok(Conditional {
            condition,
            body: block,
            else_if_condition,
            else_if_body,
            else_body: self.parse_else()?,
        })
    }

    fn parse_else_if(&mut self) -> Result<(Vec<Expression>, Vec<Block>)> {
        let mut conditions = Vec::new();
        let mut blocks = Vec::new();
        while self.next_if_type(Types::KEYWORD(Keyword::ELSE)).is_some()
            && self.next_if_type(Types::KEYWORD(Keyword::IF)).is_some()
        {
            conditions.push(self.parse_expression(vec![Types::DELIMITER(Delimiter::LBRACE)])?);
            blocks.push(self.parse_scoped_block()?);
        }
        Ok((conditions, blocks))
    }

    fn parse_else(&mut self) -> Result<Option<Block>> {
        if self.next_if_type(Types::KEYWORD(Keyword::ELSE)).is_some() {
            return self.parse_scoped_block().map(|b| Some(b));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use lexer::{lexer::Lexer, types::Datatype};

    use crate::nodes::{ASTNodes, Block, Expression, Function, Literal, Return};

    use super::*;

    #[test]
    fn test_parse_if() {
        let mut lexer = Lexer::new("func main() u32 { if true { return 1 } }");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Function(Function {
                name: "main".to_string(),
                args: vec![],
                return_type: Some(Datatype::U32),
                body: Block {
                    body: vec![ASTNodes::Conditional(Conditional {
                        condition: Expression::Simple {
                            left: Box::new(ASTNodes::Literal(Literal {
                                value: "1".to_string(),
                                r#type: lexer::types::Types::BOOL
                            })),
                            right: None,
                            operator: None
                        },
                        body: Block {
                            body: vec![ASTNodes::Return(Return {
                                value: Some(Expression::Simple {
                                    left: Box::new(ASTNodes::Literal(Literal {
                                        value: "1".to_string(),
                                        r#type: lexer::types::Types::NUMBER
                                    })),
                                    right: None,
                                    operator: None
                                })
                            })]
                        },
                        else_if_condition: vec![],
                        else_if_body: vec![],
                        else_body: None,
                    })]
                },
            })]
        )
    }

    #[test]
    fn test_parse_if_else() {
        let mut lexer = Lexer::new("func main() u32 { if true { return 1 } else { return 2 } }");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Function(Function {
                name: "main".to_string(),
                args: vec![],
                return_type: Some(Datatype::U32),
                body: Block {
                    body: vec![ASTNodes::Conditional(Conditional {
                        condition: Expression::Simple {
                            left: Box::new(ASTNodes::Literal(Literal {
                                value: "1".to_string(),
                                r#type: lexer::types::Types::BOOL
                            })),
                            right: None,
                            operator: None
                        },
                        body: Block {
                            body: vec![ASTNodes::Return(Return {
                                value: Some(Expression::Simple {
                                    left: Box::new(ASTNodes::Literal(Literal {
                                        value: "1".to_string(),
                                        r#type: lexer::types::Types::NUMBER
                                    })),
                                    right: None,
                                    operator: None
                                })
                            })]
                        },
                        else_if_condition: vec![],
                        else_if_body: vec![],
                        else_body: Some(Block {
                            body: vec![ASTNodes::Return(Return {
                                value: Some(Expression::Simple {
                                    left: Box::new(ASTNodes::Literal(Literal {
                                        value: "2".to_string(),
                                        r#type: lexer::types::Types::NUMBER
                                    })),
                                    right: None,
                                    operator: None
                                })
                            })]
                        }),
                    })]
                },
            })]
        )
    }

    #[test]
    fn test_parse_if_else_if() {
        let mut lexer = Lexer::new(
            "func main() u32 { if true { return 1 } else if false { return 2 } else if true { return 3 } else { return 4 } }",
        );
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Function(Function {
                name: "main".to_string(),
                args: vec![],
                return_type: Some(Datatype::U32),
                body: Block {
                    body: vec![ASTNodes::Conditional(Conditional {
                        condition: Expression::Simple {
                            left: Box::new(ASTNodes::Literal(Literal {
                                value: "1".to_string(),
                                r#type: lexer::types::Types::BOOL
                            })),
                            right: None,
                            operator: None
                        },
                        body: Block {
                            body: vec![ASTNodes::Return(Return {
                                value: Some(Expression::Simple {
                                    left: Box::new(ASTNodes::Literal(Literal {
                                        value: "1".to_string(),
                                        r#type: lexer::types::Types::NUMBER
                                    })),
                                    right: None,
                                    operator: None
                                })
                            })]
                        },
                        else_if_condition: vec![
                            Expression::Simple {
                                left: Box::new(ASTNodes::Literal(Literal {
                                    value: "0".to_string(),
                                    r#type: lexer::types::Types::BOOL
                                })),
                                right: None,
                                operator: None
                            },
                            Expression::Simple {
                                left: Box::new(ASTNodes::Literal(Literal {
                                    value: "1".to_string(),
                                    r#type: lexer::types::Types::BOOL
                                })),
                                right: None,
                                operator: None
                            }
                        ],
                        else_if_body: vec![
                            Block {
                                body: vec![ASTNodes::Return(Return {
                                    value: Some(Expression::Simple {
                                        left: Box::new(ASTNodes::Literal(Literal {
                                            value: "2".to_string(),
                                            r#type: lexer::types::Types::NUMBER
                                        })),
                                        right: None,
                                        operator: None
                                    })
                                })]
                            },
                            Block {
                                body: vec![ASTNodes::Return(Return {
                                    value: Some(Expression::Simple {
                                        left: Box::new(ASTNodes::Literal(Literal {
                                            value: "3".to_string(),
                                            r#type: lexer::types::Types::NUMBER
                                        })),
                                        right: None,
                                        operator: None
                                    })
                                })]
                            }
                        ],
                        else_body: Some(Block {
                            body: vec![ASTNodes::Return(Return {
                                value: Some(Expression::Simple {
                                    left: Box::new(ASTNodes::Literal(Literal {
                                        value: "4".to_string(),
                                        r#type: lexer::types::Types::NUMBER
                                    })),
                                    right: None,
                                    operator: None
                                })
                            })]
                        }),
                    })]
                },
            })]
        )
    }
}

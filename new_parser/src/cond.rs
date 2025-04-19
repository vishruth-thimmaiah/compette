use lexer::types::{Delimiter, Keyword, Types};

use crate::{Parser, Result, nodes::Conditional};

impl Parser {
    pub(crate) fn parse_if(&mut self) -> Result<Conditional> {
        let condition = self.parse_expression(vec![Types::DELIMITER(Delimiter::LBRACE)])?;
        let block = self.parse_scoped_block()?;

        Ok(Conditional::If {
            condition,
            body: block,
            else_body: self.parse_else()?.map(|b| Box::new(b)),
        })
    }

    fn parse_else(&mut self) -> Result<Option<Conditional>> {
        if self.next_if_type(Types::KEYWORD(Keyword::ELSE)).is_none() {
            return Ok(None);
        }
        if self.next_if_type(Types::KEYWORD(Keyword::IF)).is_some() {
            return self.parse_if().map(|b| Some(b));
        } else {
            return self
                .parse_scoped_block()
                .map(|b| Some(Conditional::Else { body: b }));
        }
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
                    body: vec![ASTNodes::Conditional(Conditional::If {
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
                    body: vec![ASTNodes::Conditional(Conditional::If {
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
                        else_body: Some(Box::new(Conditional::Else {
                            body: Block {
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
                            }
                        })),
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
                    body: vec![ASTNodes::Conditional(Conditional::If {
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
                        else_body: Some(Box::new(Conditional::If {
                            condition: Expression::Simple {
                                left: Box::new(ASTNodes::Literal(Literal {
                                    value: "0".to_string(),
                                    r#type: lexer::types::Types::BOOL
                                })),
                                right: None,
                                operator: None
                            },
                            body: Block {
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
                            else_body: Some(Box::new(Conditional::If {
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
                                                value: "3".to_string(),
                                                r#type: lexer::types::Types::NUMBER
                                            })),
                                            right: None,
                                            operator: None
                                        })
                                    })]
                                },
                                else_body: Some(Box::new(Conditional::Else {
                                    body: Block {
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
                                    }
                                }))
                            }))
                        }))
                    })]
                },
            })]
        )
    }
}

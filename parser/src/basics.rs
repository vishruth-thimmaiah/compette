use lexer::types::{Datatype, Delimiter, Operator, Types};

use crate::nodes::{ASTNodes, ArrayIndex, Attr, Method, Variable};

use super::{Parser, ParserError, Result};

impl Parser {
    pub(crate) fn parse_datatype(&mut self) -> Result<Datatype> {
        let token = self.next().ok_or(ParserError::default())?;
        let mut dt = if let Types::DATATYPE(dt) = token.r#type {
            dt
        } else if let Types::IDENTIFIER = token.r#type {
            Datatype::CUSTOM(token.value.unwrap())
        } else {
            return Err(ParserError::default());
        };

        // A temporary solution for parsing casts to simd
        if let Datatype::SIMD(_, _) = dt {
            self.next_with_type(Types::OPERATOR(Operator::LESSER))?;
            let dt = self.parse_datatype()?;
            self.next_with_type(Types::DELIMITER(Delimiter::COMMA))?;
            let size = self
                .next()
                .unwrap()
                .value
                .unwrap()
                .parse::<usize>()
                .unwrap();
            self.next_with_type(Types::OPERATOR(Operator::GREATER))?;
            return Ok(Datatype::SIMD(Box::new(dt), size));
        }

        while self
            .next_if_type(Types::DELIMITER(Delimiter::LBRACKET))
            .is_some()
        {
            self.next_with_type(Types::DELIMITER(Delimiter::RBRACKET))?;
            dt = Datatype::NARRAY(Box::new(dt), 0);
        }
        Ok(dt)
    }

    pub(crate) fn parse_variable(&mut self) -> Result<Variable> {
        let ident = self.next_with_type(Types::IDENTIFIER)?;
        let name = ident.value.unwrap();
        Ok(Variable { name })
    }

    /// Returns a variable, attribute or method call
    pub(crate) fn parse_complex_variable(&mut self) -> Result<ASTNodes> {
        let mut parent = if self.current_if_type(Types::IDENTIFIER_FUNC).is_some() {
            ASTNodes::FunctionCall(self.parse_function_call()?)
        } else {
            ASTNodes::Variable(Variable {
                name: self.current_with_type(Types::IDENTIFIER)?.value.unwrap(),
            })
        };

        if self.peek_if_type(Types::OPERATOR(Operator::PATH)).is_some() {
            return self
                .parse_import_call()
                .map(|call| ASTNodes::ImportCall(call));
        }

        while self.next_if_type(Types::OPERATOR(Operator::DOT)).is_some() {
            parent = if self.next_if_type(Types::IDENTIFIER_FUNC).is_some() {
                let method = self.parse_function_call()?;
                ASTNodes::Method(Method {
                    func: method,
                    parent: Box::new(parent),
                })
            } else {
                ASTNodes::Attr(Attr {
                    name: self.parse_variable()?,
                    parent: Box::new(parent),
                })
            };
        }

        while self
            .next_if_type(Types::DELIMITER(Delimiter::LBRACKET))
            .is_some()
        {
            parent = ASTNodes::ArrayIndex(ArrayIndex {
                array_var: Box::new(parent),
                index: self.parse_expression(vec![Types::DELIMITER(Delimiter::RBRACKET)])?,
            });
            self.next_with_type(Types::DELIMITER(Delimiter::RBRACKET))?;
        }

        Ok(parent)
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::{Expression, FunctionCall, Literal};

    use super::*;
    use lexer::lexer::Lexer;

    #[test]
    fn test_parse_datatype() {
        let mut lexer = Lexer::new("u32 ");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse_datatype().unwrap();
        assert_eq!(ast, Datatype::U32);
    }

    fn test_parse_array_datatype() {
        let mut lexer = Lexer::new("u32[]");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse_datatype().unwrap();
        assert_eq!(ast, Datatype::NARRAY(Box::new(Datatype::U32), 0));
    }

    #[test]
    fn test_parse_custom_datatype() {
        let mut lexer = Lexer::new("Test ");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse_datatype().unwrap();
        assert_eq!(ast, Datatype::CUSTOM("Test".to_string()));
    }

    #[test]
    fn test_parse_method_call() {
        let mut lexer = Lexer::new("Test.test()");
        let mut parser = Parser::new(lexer.tokenize());
        parser.next();
        let ast = parser.parse_complex_variable().unwrap();
        assert_eq!(
            ast,
            ASTNodes::Method(Method {
                func: FunctionCall {
                    name: "test".to_string(),
                    args: vec![]
                },
                parent: Box::new(ASTNodes::Variable(Variable {
                    name: "Test".to_string()
                }))
            })
        );
    }

    #[test]
    fn test_parse_attr() {
        let mut lexer = Lexer::new("Test.test ");
        let mut parser = Parser::new(lexer.tokenize());
        parser.next();
        let ast = parser.parse_complex_variable().unwrap();
        assert_eq!(
            ast,
            ASTNodes::Attr(Attr {
                name: Variable {
                    name: "test".to_string()
                },
                parent: Box::new(ASTNodes::Variable(Variable {
                    name: "Test".to_string()
                }))
            })
        );
    }

    #[test]
    fn test_parse_nested_method_call() {
        let mut lexer = Lexer::new("test().test2()");
        let mut parser = Parser::new(lexer.tokenize());
        parser.next();
        let ast = parser.parse_complex_variable().unwrap();
        assert_eq!(
            ast,
            ASTNodes::Method(Method {
                func: FunctionCall {
                    name: "test2".to_string(),
                    args: vec![]
                },
                parent: Box::new(ASTNodes::FunctionCall(FunctionCall {
                    name: "test".to_string(),
                    args: vec![]
                }))
            })
        );
    }

    #[test]
    fn test_parse_complex_call() {
        let mut lexer = Lexer::new("Test.test().test2(5, 3).test3.test4.test5(4)");
        let mut parser = Parser::new(lexer.tokenize());
        parser.next();
        let ast = parser.parse_complex_variable().unwrap();
        assert_eq!(
            ast,
            ASTNodes::Method(Method {
                func: FunctionCall {
                    name: "test5".to_string(),
                    args: vec![Expression::Simple {
                        left: Box::new(ASTNodes::Literal(Literal {
                            value: "4".to_string(),
                            r#type: Types::NUMBER
                        })),
                        right: None,
                        operator: None
                    }]
                },
                parent: Box::new(ASTNodes::Attr(Attr {
                    name: Variable {
                        name: "test4".to_string()
                    },
                    parent: Box::new(ASTNodes::Attr(Attr {
                        name: Variable {
                            name: "test3".to_string()
                        },
                        parent: Box::new(ASTNodes::Method(Method {
                            func: FunctionCall {
                                name: "test2".to_string(),
                                args: vec![
                                    Expression::Simple {
                                        left: Box::new(ASTNodes::Literal(Literal {
                                            value: "5".to_string(),
                                            r#type: Types::NUMBER
                                        })),
                                        right: None,
                                        operator: None
                                    },
                                    Expression::Simple {
                                        left: Box::new(ASTNodes::Literal(Literal {
                                            value: "3".to_string(),
                                            r#type: Types::NUMBER
                                        })),
                                        right: None,
                                        operator: None
                                    }
                                ]
                            },
                            parent: Box::new(ASTNodes::Method(Method {
                                func: FunctionCall {
                                    name: "test".to_string(),
                                    args: vec![]
                                },
                                parent: Box::new(ASTNodes::Variable(Variable {
                                    name: "Test".to_string()
                                })),
                            })),
                        })),
                    })),
                })),
            })
        );
    }

    #[test]
    fn test_parse_complex_call_2() {
        let mut lexer = Lexer::new("test().test2(5, 3).test3.test4.test5(4)");
        let mut parser = Parser::new(lexer.tokenize());
        parser.next();
        let ast = parser.parse_complex_variable().unwrap();
        assert_eq!(
            ast,
            ASTNodes::Method(Method {
                func: FunctionCall {
                    name: "test5".to_string(),
                    args: vec![Expression::Simple {
                        left: Box::new(ASTNodes::Literal(Literal {
                            value: "4".to_string(),
                            r#type: Types::NUMBER
                        })),
                        right: None,
                        operator: None
                    }]
                },
                parent: Box::new(ASTNodes::Attr(Attr {
                    name: Variable {
                        name: "test4".to_string()
                    },
                    parent: Box::new(ASTNodes::Attr(Attr {
                        name: Variable {
                            name: "test3".to_string()
                        },
                        parent: Box::new(ASTNodes::Method(Method {
                            func: FunctionCall {
                                name: "test2".to_string(),
                                args: vec![
                                    Expression::Simple {
                                        left: Box::new(ASTNodes::Literal(Literal {
                                            value: "5".to_string(),
                                            r#type: Types::NUMBER
                                        })),
                                        right: None,
                                        operator: None
                                    },
                                    Expression::Simple {
                                        left: Box::new(ASTNodes::Literal(Literal {
                                            value: "3".to_string(),
                                            r#type: Types::NUMBER
                                        })),
                                        right: None,
                                        operator: None
                                    }
                                ]
                            },
                            parent: Box::new(ASTNodes::FunctionCall(FunctionCall {
                                name: "test".to_string(),
                                args: vec![]
                            },)),
                        })),
                    })),
                })),
            })
        );
    }

    #[test]
    fn test_parse_complex_call_3() {
        let mut lexer = Lexer::new("test[0]");
        let mut parser = Parser::new(lexer.tokenize());
        parser.next();
        let ast = parser.parse_complex_variable().unwrap();
        assert_eq!(
            ast,
            ASTNodes::ArrayIndex(ArrayIndex {
                array_var: Box::new(ASTNodes::Variable(Variable {
                    name: "test".to_string()
                })),
                index: Expression::Simple {
                    left: Box::new(ASTNodes::Literal(Literal {
                        value: "0".to_string(),
                        r#type: Types::NUMBER
                    })),
                    right: None,
                    operator: None
                }
            })
        );
    }

    #[test]
    fn test_parse_complex_call_4() {
        let mut lexer = Lexer::new("test[0][1]");
        let mut parser = Parser::new(lexer.tokenize());
        parser.next();
        let ast = parser.parse_complex_variable().unwrap();
        assert_eq!(
            ast,
            ASTNodes::ArrayIndex(ArrayIndex {
                array_var: Box::new(ASTNodes::ArrayIndex(ArrayIndex {
                    array_var: Box::new(ASTNodes::Variable(Variable {
                        name: "test".to_string(),
                    })),
                    index: Expression::Simple {
                        left: Box::new(ASTNodes::Literal(Literal {
                            value: "0".to_string(),
                            r#type: Types::NUMBER
                        })),
                        right: None,
                        operator: None
                    },
                })),
                index: Expression::Simple {
                    left: Box::new(ASTNodes::Literal(Literal {
                        value: "1".to_string(),
                        r#type: Types::NUMBER
                    })),
                    right: None,
                    operator: None
                }
            })
        );
    }
}

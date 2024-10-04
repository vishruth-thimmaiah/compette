#[cfg(test)]
mod tests {
    use crate::lexer::{
        lexer::{Lexer, Token},
        types::Types::*,
    };

    use super::super::{nodes::*, parser};

    #[test]
    fn test_parser() {
        let contents = r#"
        let a = 1
        let b = 2

        if a == 1 {
            let c = 1
        } else if a == 2 {
            let c = 2
        } else {
            let c = 3
        }

        loop a == 5 {
            let c = c + 1
        }

        func add(x, y) {
            let q = x + y
        }

        add(a, b)
        "#;

        let req_result: Vec<Box<dyn ParserType>> = vec![
            Box::new(AssignmentParserNode {
                var_name: "a".to_string(),
                value: Box::new(ExpressionParserNode {
                    left: Token {
                        line: 0,
                        column: 0,
                        r#type: NUMBER,
                        value: Some("1".to_string()),
                    },
                    right: None,
                    operator: None,
                }),
            }),
            Box::new(AssignmentParserNode {
                var_name: "b".to_string(),
                value: Box::new(ExpressionParserNode {
                    left: Token {
                        line: 0,
                        column: 0,
                        r#type: NUMBER,
                        value: Some("2".to_string()),
                    },
                    right: None,
                    operator: None,
                }),
            }),
            Box::new(ConditionalIfParserNode {
                condition: Box::new(ExpressionParserNode {
                    left: Token {
                        line: 0,
                        column: 0,
                        r#type: IDENTIFIER,
                        value: Some("a".to_string()),
                    },
                    right: Some(Box::new(ExpressionParserNode {
                        left: Token {
                            line: 0,
                            column: 0,
                            r#type: NUMBER,
                            value: Some("1".to_string()),
                        },
                        right: None,
                        operator: None,
                    })),
                    operator: Some(EQUAL),
                }),
                body: vec![Box::new(AssignmentParserNode {
                    var_name: "c".to_string(),
                    value: Box::new(ExpressionParserNode {
                        left: Token {
                            line: 0,
                            column: 0,
                            r#type: NUMBER,
                            value: Some("1".to_string()),
                        },
                        right: None,
                        operator: None,
                    }),
                })],
                else_if_body: vec![ConditionalElseIfParserNode {
                    condition: Box::new(ExpressionParserNode {
                        left: Token {
                            line: 0,
                            column: 0,
                            r#type: IDENTIFIER,
                            value: Some("a".to_string()),
                        },
                        right: Some(Box::new(ExpressionParserNode {
                            left: Token {
                                line: 0,
                                column: 0,
                                r#type: NUMBER,
                                value: Some("2".to_string()),
                            },
                            right: None,
                            operator: None,
                        })),
                        operator: Some(EQUAL),
                    }),
                    body: vec![Box::new(AssignmentParserNode {
                        var_name: "c".to_string(),
                        value: Box::new(ExpressionParserNode {
                            left: Token {
                                line: 0,
                                column: 0,
                                r#type: NUMBER,
                                value: Some("2".to_string()),
                            },
                            right: None,
                            operator: None,
                        }),
                    })],
                }],
                else_body: Some(ConditionalElseParserNode {
                    body: vec![Box::new(AssignmentParserNode {
                        var_name: "c".to_string(),
                        value: Box::new(ExpressionParserNode {
                            left: Token {
                                line: 0,
                                column: 0,
                                r#type: NUMBER,
                                value: Some("3".to_string()),
                            },
                            right: None,
                            operator: None,
                        }),
                    })],
                }),
            }),
            Box::new(LoopParserNode {
                condition: Box::new(ExpressionParserNode {
                    left: Token {
                        line: 0,
                        column: 0,
                        r#type: IDENTIFIER,
                        value: Some("a".to_string()),
                    },
                    right: Some(Box::new(ExpressionParserNode {
                        left: Token {
                            line: 0,
                            column: 0,
                            r#type: NUMBER,
                            value: Some("5".to_string()),
                        },
                        right: None,
                        operator: None,
                    })),
                    operator: Some(EQUAL),
                }),
                body: vec![Box::new(AssignmentParserNode {
                    var_name: "c".to_string(),
                    value: Box::new(ExpressionParserNode {
                        left: Token {
                            line: 0,
                            column: 0,
                            r#type: IDENTIFIER,
                            value: Some("c".to_string()),
                        },
                        right: Some(Box::new(ExpressionParserNode {
                            left: Token {
                                line: 0,
                                column: 0,
                                r#type: NUMBER,
                                value: Some("1".to_string()),
                            },
                            right: None,
                            operator: None,
                        })),
                        operator: Some(ASSIGN),
                    }),
                })],
            }),
            Box::new(FunctionParserNode {
                func_name: "add".to_string(),
                args: vec!["x".to_string(), "y".to_string()],
                body: vec![Box::new(AssignmentParserNode {
                    var_name: "q".to_string(),
                    value: Box::new(ExpressionParserNode {
                        left: Token {
                            line: 0,
                            column: 0,
                            r#type: IDENTIFIER,
                            value: Some("x".to_string()),
                        },
                        right: Some(Box::new(ExpressionParserNode {
                            left: Token {
                                line: 0,
                                column: 0,
                                r#type: IDENTIFIER,
                                value: Some("y".to_string()),
                            },
                            right: None,
                            operator: None,
                        })),
                        operator: Some(PLUS),
                    }),
                })],
            }),
            Box::new(FunctionCallParserNode {
                func_name: "add".to_string(),
                args: vec!["a".to_string(), "b".to_string()],
            }),
        ];

        let lexer_output = Lexer::new(contents).tokenize();

        let mut parser = parser::Parser::new(lexer_output);
        let result = parser.parse();

        assert_eq!(result.len(), req_result.len());

        for i in 0..result.len() {
            //TODO: Check if the actual structs are the same
            assert_eq!(result[i].get_type(), req_result[i].get_type());
        }
    }

    #[test]
    fn test_function_ast() {
        let contents = r#"
        let a = 1
        a = 4
        let b = 2

        func add(x, y) {
            let q = x + y
        }

        add(a, b)
        "#;

        let req_result: Vec<Box<dyn ParserType>> = vec![
            Box::new(AssignmentParserNode {
                var_name: "a".to_string(),
                value: Box::new(ExpressionParserNode {
                    left: Token {
                        line: 0,
                        column: 0,
                        r#type: NUMBER,
                        value: Some("1".to_string()),
                    },
                    right: None,
                    operator: None,
                }),
            }),
            Box::new(VariableCallParserNode {
                var_name: "a".to_string(),
                rhs: Box::new(ExpressionParserNode {
                    left: Token {
                        r#type: NUMBER,
                        value: Some("4".to_string()),
                        line: 0,
                        column: 0,
                    },
                    right: None,
                    operator: None,
                }),
            }),
            Box::new(AssignmentParserNode {
                var_name: "b".to_string(),
                value: Box::new(ExpressionParserNode {
                    left: Token {
                        line: 0,
                        column: 0,
                        r#type: NUMBER,
                        value: Some("2".to_string()),
                    },
                    right: None,
                    operator: None,
                }),
            }),
            Box::new(FunctionParserNode {
                func_name: "add".to_string(),
                args: vec!["x".to_string(), "y".to_string()],
                body: vec![Box::new(AssignmentParserNode {
                    var_name: "q".to_string(),
                    value: Box::new(ExpressionParserNode {
                        left: Token {
                            line: 0,
                            column: 0,
                            r#type: IDENTIFIER,
                            value: Some("x".to_string()),
                        },
                        right: Some(Box::new(ExpressionParserNode {
                            left: Token {
                                line: 0,
                                column: 0,
                                r#type: IDENTIFIER,
                                value: Some("y".to_string()),
                            },
                            right: None,
                            operator: None,
                        })),
                        operator: Some(PLUS),
                    }),
                })],
            }),
            Box::new(FunctionCallParserNode {
                func_name: "add".to_string(),
                args: vec!["a".to_string(), "b".to_string()],
            }),
        ];

        let lexer_output = Lexer::new(contents).tokenize();

        let mut parser = parser::Parser::new(lexer_output);
        let result = parser.parse();

        assert_eq!(result.len(), req_result.len());

        for i in 0..result.len() {
            //TODO: Check if the actual structs are the same
            assert_eq!(result[i].get_type(), req_result[i].get_type());
        }
    }
}

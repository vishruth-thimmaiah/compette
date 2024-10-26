#[cfg(test)]
mod tests {
    use crate::lexer::{
        lexer::Lexer,
        types::{
            Types::{self, *},
            DATATYPE, OPERATOR,
        },
    };

    use super::super::{nodes::*, parser};

    #[test]
    fn test_parser() {
        let contents = r#"
        let u32 a = 1
        let u32 b = 2

        if a == 1 {
            let u32 c = 1
        } else if a == 2 {
            let u32 c = 2
        } else {
            let u32 c = 3
        }

        loop a == 5 {
            let u32 c = c + 1
        }

        func add(x u32, y u32) u32 {
            let u32 q = x + y
            return q
        }

        add(a, b)
        "#;

        let req_result: Vec<Box<dyn ParserType>> = vec![
            Box::new(AssignmentParserNode {
                var_name: "a".to_string(),
                var_type: DATATYPE::U32,
                is_mutable: false,
                value: Box::new(ExpressionParserNode {
                    left: Box::new(ValueParserNode {
                        r#type: NUMBER,
                        value: "1".to_string(),
                    }),
                    right: None,
                    operator: None,
                }),
            }),
            Box::new(AssignmentParserNode {
                var_name: "b".to_string(),
                var_type: DATATYPE::U32,
                is_mutable: false,
                value: Box::new(ExpressionParserNode {
                    left: Box::new(ValueParserNode {
                        r#type: NUMBER,
                        value: "2".to_string(),
                    }),
                    right: None,
                    operator: None,
                }),
            }),
            Box::new(ConditionalIfParserNode {
                condition: Box::new(ExpressionParserNode {
                    left: Box::new(ValueParserNode {
                        r#type: IDENTIFIER,
                        value: "a".to_string(),
                    }),
                    right: Some(Box::new(ExpressionParserNode {
                        left: Box::new(ValueParserNode {
                            r#type: NUMBER,
                            value: "1".to_string(),
                        }),
                        right: None,
                        operator: None,
                    })),
                    operator: Some(OPERATOR::EQUAL),
                }),
                body: vec![Box::new(AssignmentParserNode {
                    var_name: "c".to_string(),
                    var_type: DATATYPE::U32,
                    is_mutable: false,
                    value: Box::new(ExpressionParserNode {
                        left: Box::new(ValueParserNode {
                            r#type: NUMBER,
                            value: "1".to_string(),
                        }),
                        right: None,
                        operator: None,
                    }),
                })],
                else_if_body: vec![ConditionalElseIfParserNode {
                    condition: Box::new(ExpressionParserNode {
                        left: Box::new(ValueParserNode {
                            r#type: IDENTIFIER,
                            value: "a".to_string(),
                        }),
                        right: Some(Box::new(ExpressionParserNode {
                            left: Box::new(ValueParserNode {
                                r#type: NUMBER,
                                value: "2".to_string(),
                            }),
                            right: None,
                            operator: None,
                        })),
                        operator: Some(OPERATOR::EQUAL),
                    }),
                    body: vec![Box::new(AssignmentParserNode {
                        var_name: "c".to_string(),
                        is_mutable: false,
                        var_type: DATATYPE::U32,
                        value: Box::new(ExpressionParserNode {
                            left: Box::new(ValueParserNode {
                                r#type: NUMBER,
                                value: "2".to_string(),
                            }),
                            right: None,
                            operator: None,
                        }),
                    })],
                }],
                else_body: Some(ConditionalElseParserNode {
                    body: vec![Box::new(AssignmentParserNode {
                        var_type: DATATYPE::U32,
                        var_name: "c".to_string(),
                        is_mutable: false,
                        value: Box::new(ExpressionParserNode {
                            left: Box::new(ValueParserNode {
                                r#type: NUMBER,
                                value: "3".to_string(),
                            }),
                            right: None,
                            operator: None,
                        }),
                    })],
                }),
            }),
            Box::new(LoopParserNode {
                condition: Box::new(ExpressionParserNode {
                    left: Box::new(ValueParserNode {
                        r#type: IDENTIFIER,
                        value: "a".to_string(),
                    }),
                    right: Some(Box::new(ExpressionParserNode {
                        left: Box::new(ValueParserNode {
                            r#type: NUMBER,
                            value: "5".to_string(),
                        }),
                        right: None,
                        operator: None,
                    })),
                    operator: Some(OPERATOR::EQUAL),
                }),
                body: vec![Box::new(AssignmentParserNode {
                    var_name: "c".to_string(),
                    var_type: DATATYPE::U32,
                    is_mutable: false,
                    value: Box::new(ExpressionParserNode {
                        left: Box::new(ValueParserNode {
                            r#type: IDENTIFIER,
                            value: "c".to_string(),
                        }),
                        right: Some(Box::new(ExpressionParserNode {
                            left: Box::new(ValueParserNode {
                                r#type: NUMBER,
                                value: "1".to_string(),
                            }),
                            right: None,
                            operator: None,
                        })),
                        operator: Some(OPERATOR::ASSIGN),
                    }),
                })],
            }),
            Box::new(FunctionParserNode {
                func_name: "add".to_string(),
                args: vec![
                    ("x".to_string(), DATATYPE::U32),
                    ("y".to_string(), DATATYPE::U32),
                ],
                return_type: Some(DATATYPE::U32),
                body: vec![
                    Box::new(AssignmentParserNode {
                        var_name: "q".to_string(),
                        var_type: DATATYPE::U32,
                        is_mutable: false,
                        value: Box::new(ExpressionParserNode {
                            left: Box::new(ValueParserNode {
                                r#type: IDENTIFIER,
                                value: "x".to_string(),
                            }),
                            right: Some(Box::new(ExpressionParserNode {
                                left: Box::new(ValueParserNode {
                                    r#type: IDENTIFIER,
                                    value: "y".to_string(),
                                }),
                                right: None,
                                operator: None,
                            })),
                            operator: Some(OPERATOR::PLUS),
                        }),
                    }),
                    Box::new(ReturnNode {
                        return_value: Box::new(ExpressionParserNode {
                            left: Box::new(ValueParserNode {
                                r#type: IDENTIFIER,
                                value: "a".to_string(),
                            }),
                            right: None,
                            operator: None,
                        }),
                    }),
                ],
            }),
            Box::new(FunctionCallParserNode {
                func_name: "add".to_string(),
                args: vec![
                    ExpressionParserNode {
                        left: Box::new(ValueParserNode {
                            value: "a".to_string(),
                            r#type: Types::IDENTIFIER,
                        }),
                        right: None,
                        operator: None,
                    },
                    ExpressionParserNode {
                        left: Box::new(ValueParserNode {
                            value: "b".to_string(),
                            r#type: Types::IDENTIFIER,
                        }),
                        right: None,
                        operator: None,
                    },
                ],
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
        let u32 a = 1
        a = 4
        let u32! b = 2

        func add(x u32, y u32) u32 {
            let u32 q = x + y
        }

        add(a, b)
        "#;

        let req_result: Vec<Box<dyn ParserType>> = vec![
            Box::new(AssignmentParserNode {
                var_name: "a".to_string(),
                var_type: DATATYPE::U32,
                is_mutable: false,
                value: Box::new(ExpressionParserNode {
                    left: Box::new(ValueParserNode {
                        r#type: NUMBER,
                        value: "1".to_string(),
                    }),
                    right: None,
                    operator: None,
                }),
            }),
            Box::new(VariableCallParserNode {
                var_name: Box::new(ValueParserNode {
                    r#type: IDENTIFIER,
                    value: "a".to_string(),
                }),
                rhs: Box::new(ExpressionParserNode {
                    left: Box::new(ValueParserNode {
                        r#type: NUMBER,
                        value: "4".to_string(),
                    }),
                    right: None,
                    operator: None,
                }),
            }),
            Box::new(AssignmentParserNode {
                var_name: "b".to_string(),
                var_type: DATATYPE::U32,
                is_mutable: true,
                value: Box::new(ExpressionParserNode {
                    left: Box::new(ValueParserNode {
                        r#type: NUMBER,
                        value: "2".to_string(),
                    }),
                    right: None,
                    operator: None,
                }),
            }),
            Box::new(FunctionParserNode {
                func_name: "add".to_string(),
                return_type: Some(DATATYPE::U32),
                args: vec![
                    ("x".to_string(), DATATYPE::U32),
                    ("y".to_string(), DATATYPE::U32),
                ],
                body: vec![Box::new(AssignmentParserNode {
                    var_name: "q".to_string(),
                    var_type: DATATYPE::U32,
                    is_mutable: false,
                    value: Box::new(ExpressionParserNode {
                        left: Box::new(ValueParserNode {
                            r#type: IDENTIFIER,
                            value: "x".to_string(),
                        }),
                        right: Some(Box::new(ExpressionParserNode {
                            left: Box::new(ValueParserNode {
                                r#type: IDENTIFIER,
                                value: "y".to_string(),
                            }),
                            right: None,
                            operator: None,
                        })),
                        operator: Some(OPERATOR::PLUS),
                    }),
                })],
            }),
            Box::new(FunctionCallParserNode {
                func_name: "add".to_string(),
                args: vec![
                    ExpressionParserNode {
                        left: Box::new(ValueParserNode {
                            value: "a".to_string(),
                            r#type: Types::IDENTIFIER,
                        }),
                        right: None,
                        operator: None,
                    },
                    ExpressionParserNode {
                        left: Box::new(ValueParserNode {
                            value: "b".to_string(),
                            r#type: Types::IDENTIFIER,
                        }),
                        right: None,
                        operator: None,
                    },
                ],
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
    fn test_function_mult_func() {
        let contents = r#"
        func num() u32 {
            return 3
        }

        func main() u32 {
            let u32 a = 2
            let u32 b = num()
            return a * b
        }
        "#;

        let req_result: Vec<Box<dyn ParserType>> = vec![
            Box::new(FunctionParserNode {
                func_name: "num".to_string(),
                args: vec![],
                return_type: Some(DATATYPE::U32),
                body: vec![Box::new(ReturnNode {
                    return_value: Box::new(ExpressionParserNode {
                        left: Box::new(ValueParserNode {
                            r#type: IDENTIFIER,
                            value: "3".to_string(),
                        }),
                        right: None,
                        operator: None,
                    }),
                })],
            }),
            Box::new(FunctionParserNode {
                func_name: "main".to_string(),
                args: vec![],
                return_type: Some(DATATYPE::U32),
                body: vec![
                    Box::new(AssignmentParserNode {
                        var_name: "a".to_string(),
                        var_type: DATATYPE::U32,
                        is_mutable: false,
                        value: Box::new(ExpressionParserNode {
                            left: Box::new(ValueParserNode {
                                r#type: NUMBER,
                                value: "2".to_string(),
                            }),
                            right: None,
                            operator: None,
                        }),
                    }),
                    Box::new(AssignmentParserNode {
                        var_name: "b".to_string(),
                        var_type: DATATYPE::U32,
                        is_mutable: false,
                        value: Box::new(FunctionCallParserNode {
                            func_name: "num".to_string(),
                            args: vec![],
                        }),
                    }),
                    Box::new(ReturnNode {
                        return_value: Box::new(ExpressionParserNode {
                            left: Box::new(ValueParserNode {
                                r#type: IDENTIFIER,
                                value: "a".to_string(),
                            }),
                            right: Some(Box::new(ExpressionParserNode {
                                left: Box::new(ValueParserNode {
                                    r#type: IDENTIFIER,
                                    value: "b".to_string(),
                                }),
                                right: None,
                                operator: None,
                            })),
                            operator: Some(OPERATOR::MULTIPLY),
                        }),
                    }),
                ],
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

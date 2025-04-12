#[cfg(test)]
mod tests {
    use crate::{
        lexer::{
            lexer::Lexer,
            types::{Datatype, Operator, Types::*},
        },
        parser::Parser,
    };

    use super::super::nodes::*;

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
                return_type: Datatype::U32,
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
                return_type: Datatype::U32,
                body: vec![
                    Box::new(AssignmentParserNode {
                        var_name: "a".to_string(),
                        var_type: Datatype::U32,
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
                        var_type: Datatype::U32,
                        is_mutable: false,
                        value: Box::new(FunctionCallParserNode {
                            func_name: "num".to_string(),
                            args: vec![],
                            imported: None,
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
                            operator: Some(Operator::MULTIPLY),
                        }),
                    }),
                ],
            }),
        ];

        let lexer_output = Lexer::new(contents).tokenize();

        let mut parser = Parser::new(lexer_output);
        let result = parser.parse();

        assert_eq!(result.len(), req_result.len());

        for i in 0..result.len() {
            //TODO: Check if the actual structs are the same
            assert_eq!(result[i].get_type(), req_result[i].get_type());
        }
    }
}

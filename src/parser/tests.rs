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

        func add(x, y) {
            return x + y
        }

        add(a, b)
        "#;

        let req_result: Vec<Box<dyn ParserType>> = vec![
            Box::new(AssignmentParserNode {
                var_name: "a".to_string(),
                value: Box::new(ExpressionParserNode {
                    left: Token {
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
                body: vec![],
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

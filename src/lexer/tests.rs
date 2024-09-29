#[cfg(test)]
mod tests {

    use super::super::{lexer::*, types::Types::*};

    #[test]
    fn check_lexer() {
        let contents = r#"
        let a = 1
        let b = 2

        func add(x, y) {
            x + y
        }

        add(a, b)
        "#;

        let req_result = vec![
            Token::new(NL, None),
            Token::new(LET, None),
            Token::new(IDENTIFIER, Some("a".to_string())),
            Token::new(ASSIGN, None),
            Token::new(NUMBER, Some("1".to_string())),
            Token::new(NL, None),
            Token::new(LET, None),
            Token::new(IDENTIFIER, Some("b".to_string())),
            Token::new(ASSIGN, None),
            Token::new(NUMBER, Some("2".to_string())),
            Token::new(NL, None),
            Token::new(NL, None),
            Token::new(FUNCTION, None),
            Token::new(IDENTIFIER, Some("add".to_string())),
            Token::new(LPAREN, None),
            Token::new(IDENTIFIER, Some("x".to_string())),
            Token::new(COMMA, None),
            Token::new(IDENTIFIER, Some("y".to_string())),
            Token::new(RPAREN, None),
            Token::new(LBRACE, None),
            Token::new(NL, None),
            Token::new(IDENTIFIER, Some("x".to_string())),
            Token::new(PLUS, None),
            Token::new(IDENTIFIER, Some("y".to_string())),
            Token::new(NL, None),
            Token::new(RBRACE, None),
            Token::new(NL, None),
            Token::new(NL, None),
            Token::new(IDENTIFIER, Some("add".to_string())),
            Token::new(LPAREN, None),
            Token::new(IDENTIFIER, Some("a".to_string())),
            Token::new(COMMA, None),
            Token::new(IDENTIFIER, Some("b".to_string())),
            Token::new(RPAREN, None),
            Token::new(NL, None),
        ];
        let resutlt = Lexer::new(contents).tokenize();

        assert_eq!(req_result.len(), resutlt.len());

        for i in 0..req_result.len() {
            assert_eq!(req_result[i], resutlt[i]);
        }

    }
}

#[cfg(test)]
mod tests {

    use super::super::{lexer::*, types::Types::*};

    #[test]
    fn check_lexer() {
        let contents = r#"
        let a = 1
        let b = 2

        if a == 1 {
            c = 1
        }
        else if a != 2 {
            c = 2
        }
        else {
            c = 3
        }

        loop a == 5 {
            let c = c + 1
        }

        func add(x, y) {
            return x + y
        }

        add(a, b)
        "#;

        let req_result = vec![
            Token::new(NL, None, 0, 0),
            Token::new(LET, None, 0, 0),
            Token::new(IDENTIFIER, Some("a".to_string()), 0, 0),
            Token::new(ASSIGN, None, 0, 0),
            Token::new(NUMBER, Some("1".to_string()), 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(LET, None, 0, 0),
            Token::new(IDENTIFIER, Some("b".to_string()), 0, 0),
            Token::new(ASSIGN, None, 0, 0),
            Token::new(NUMBER, Some("2".to_string()), 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(IF, None, 0, 0),
            Token::new(IDENTIFIER, Some("a".to_string()), 0, 0),
            Token::new(EQUAL, None, 0, 0),
            Token::new(NUMBER, Some("1".to_string()), 0, 0),
            Token::new(LBRACE, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(IDENTIFIER, Some("c".to_string()), 0, 0),
            Token::new(ASSIGN, None, 0, 0),
            Token::new(NUMBER, Some("1".to_string()), 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(RBRACE, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(ELSE, None, 0, 0),
            Token::new(IF, None, 0, 0),
            Token::new(IDENTIFIER, Some("a".to_string()), 0, 0),
            Token::new(NOT_EQUAL, None, 0, 0),
            Token::new(NUMBER, Some("2".to_string()), 0, 0),
            Token::new(LBRACE, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(IDENTIFIER, Some("c".to_string()), 0, 0),
            Token::new(ASSIGN, None, 0, 0),
            Token::new(NUMBER, Some("2".to_string()), 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(RBRACE, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(ELSE, None, 0, 0),
            Token::new(LBRACE, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(IDENTIFIER, Some("c".to_string()), 0, 0),
            Token::new(ASSIGN, None, 0, 0),
            Token::new(NUMBER, Some("3".to_string()), 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(RBRACE, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(LOOP, None, 0, 0),
            Token::new(IDENTIFIER, Some("a".to_string()), 0, 0),
            Token::new(EQUAL, None, 0, 0),
            Token::new(NUMBER, Some("5".to_string()), 0, 0),
            Token::new(LBRACE, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(LET, None, 0, 0),
            Token::new(IDENTIFIER, Some("c".to_string()), 0, 0),
            Token::new(ASSIGN, None, 0, 0),
            Token::new(IDENTIFIER, Some("c".to_string()), 0, 0),
            Token::new(PLUS, None, 0, 0),
            Token::new(NUMBER, Some("1".to_string()), 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(RBRACE, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(FUNCTION, None, 0, 0),
            Token::new(IDENTIFIER, Some("add".to_string()), 0, 0),
            Token::new(LPAREN, None, 0, 0),
            Token::new(IDENTIFIER, Some("x".to_string()), 0, 0),
            Token::new(COMMA, None, 0, 0),
            Token::new(IDENTIFIER, Some("y".to_string()), 0, 0),
            Token::new(RPAREN, None, 0, 0),
            Token::new(LBRACE, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(RETURN, None, 0, 0),
            Token::new(IDENTIFIER, Some("x".to_string()), 0, 0),
            Token::new(PLUS, None, 0, 0),
            Token::new(IDENTIFIER, Some("y".to_string()), 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(RBRACE, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(IDENTIFIER, Some("add".to_string()), 0, 0),
            Token::new(LPAREN, None, 0, 0),
            Token::new(IDENTIFIER, Some("a".to_string()), 0, 0),
            Token::new(COMMA, None, 0, 0),
            Token::new(IDENTIFIER, Some("b".to_string()), 0, 0),
            Token::new(RPAREN, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(EOF, None, 0, 0),
        ];
        let resutlt = Lexer::new(contents).tokenize();

        assert_eq!(req_result.len(), resutlt.len());

        for i in 0..req_result.len() {
            assert_eq!(req_result[i].r#type, resutlt[i].r#type);
            assert_eq!(req_result[i].value, resutlt[i].value);
        }
    }
}

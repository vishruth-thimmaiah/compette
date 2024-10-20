#[cfg(test)]
mod tests {

    use crate::lexer::types::{DATATYPE, KEYWORD, OPERATOR, DELIMITER};

    use super::super::{lexer::*, types::Types::*};

    #[test]
    fn check_lexer() {
        let contents = r#"
        let u32 a = 1
        let u32 b = 2

        // let u32 c = 3

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
            let u32 c = c + 1
        }

        func add(x, y) u32 {
            return x + y
        }

        add(a, b)
        "#;

        let req_result = vec![
            Token::new(NL, None, 0, 0),
            Token::new(KEYWORD(KEYWORD::LET), None, 0, 0),
            Token::new(DATATYPE(DATATYPE::U32), None, 0, 0),
            Token::new(IDENTIFIER, Some("a".to_string()), 0, 0),
            Token::new(OPERATOR(OPERATOR::ASSIGN), None, 0, 0),
            Token::new(NUMBER, Some("1".to_string()), 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(KEYWORD(KEYWORD::LET), None, 0, 0),
            Token::new(DATATYPE(DATATYPE::U32), None, 0, 0),
            Token::new(IDENTIFIER, Some("b".to_string()), 0, 0),
            Token::new(OPERATOR(OPERATOR::ASSIGN), None, 0, 0),
            Token::new(NUMBER, Some("2".to_string()), 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(KEYWORD(KEYWORD::IF), None, 0, 0),
            Token::new(IDENTIFIER, Some("a".to_string()), 0, 0),
            Token::new(OPERATOR(OPERATOR::EQUAL), None, 0, 0),
            Token::new(NUMBER, Some("1".to_string()), 0, 0),
            Token::new(DELIMITER(DELIMITER::LBRACE), None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(IDENTIFIER, Some("c".to_string()), 0, 0),
            Token::new(OPERATOR(OPERATOR::ASSIGN), None, 0, 0),
            Token::new(NUMBER, Some("1".to_string()), 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(DELIMITER(DELIMITER::RBRACE), None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(KEYWORD(KEYWORD::ELSE), None, 0, 0),
            Token::new(KEYWORD(KEYWORD::IF), None, 0, 0),
            Token::new(IDENTIFIER, Some("a".to_string()), 0, 0),
            Token::new(OPERATOR(OPERATOR::NOT_EQUAL), None, 0, 0),
            Token::new(NUMBER, Some("2".to_string()), 0, 0),
            Token::new(DELIMITER(DELIMITER::LBRACE), None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(IDENTIFIER, Some("c".to_string()), 0, 0),
            Token::new(OPERATOR(OPERATOR::ASSIGN), None, 0, 0),
            Token::new(NUMBER, Some("2".to_string()), 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(DELIMITER(DELIMITER::RBRACE), None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(KEYWORD(KEYWORD::ELSE), None, 0, 0),
            Token::new(DELIMITER(DELIMITER::LBRACE), None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(IDENTIFIER, Some("c".to_string()), 0, 0),
            Token::new(OPERATOR(OPERATOR::ASSIGN), None, 0, 0),
            Token::new(NUMBER, Some("3".to_string()), 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(DELIMITER(DELIMITER::RBRACE), None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(KEYWORD(KEYWORD::LOOP), None, 0, 0),
            Token::new(IDENTIFIER, Some("a".to_string()), 0, 0),
            Token::new(OPERATOR(OPERATOR::EQUAL), None, 0, 0),
            Token::new(NUMBER, Some("5".to_string()), 0, 0),
            Token::new(DELIMITER(DELIMITER::LBRACE), None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(KEYWORD(KEYWORD::LET), None, 0, 0),
            Token::new(DATATYPE(DATATYPE::U32), None, 0, 0),
            Token::new(IDENTIFIER, Some("c".to_string()), 0, 0),
            Token::new(OPERATOR(OPERATOR::ASSIGN), None, 0, 0),
            Token::new(IDENTIFIER, Some("c".to_string()), 0, 0),
            Token::new(OPERATOR(OPERATOR::PLUS), None, 0, 0),
            Token::new(NUMBER, Some("1".to_string()), 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(DELIMITER(DELIMITER::RBRACE), None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(KEYWORD(KEYWORD::FUNCTION), None, 0, 0),
            Token::new(IDENTIFIER_FUNC, Some("add".to_string()), 0, 0),
            Token::new(DELIMITER(DELIMITER::LPAREN), None, 0, 0),
            Token::new(IDENTIFIER, Some("x".to_string()), 0, 0),
            Token::new(DELIMITER(DELIMITER::COMMA), None, 0, 0),
            Token::new(IDENTIFIER, Some("y".to_string()), 0, 0),
            Token::new(DELIMITER(DELIMITER::RPAREN), None, 0, 0),
            Token::new(DATATYPE(DATATYPE::U32), None, 0, 0),
            Token::new(DELIMITER(DELIMITER::LBRACE), None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(KEYWORD(KEYWORD::RETURN), None, 0, 0),
            Token::new(IDENTIFIER, Some("x".to_string()), 0, 0),
            Token::new(OPERATOR(OPERATOR::PLUS), None, 0, 0),
            Token::new(IDENTIFIER, Some("y".to_string()), 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(DELIMITER(DELIMITER::RBRACE), None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(IDENTIFIER_FUNC, Some("add".to_string()), 0, 0),
            Token::new(DELIMITER(DELIMITER::LPAREN), None, 0, 0),
            Token::new(IDENTIFIER, Some("a".to_string()), 0, 0),
            Token::new(DELIMITER(DELIMITER::COMMA), None, 0, 0),
            Token::new(IDENTIFIER, Some("b".to_string()), 0, 0),
            Token::new(DELIMITER(DELIMITER::RPAREN), None, 0, 0),
            Token::new(NL, None, 0, 0),
            Token::new(EOF, None, 0, 0),
        ];
        let result = Lexer::new(contents).tokenize();

        assert_eq!(req_result.len(), result.len());

        for i in 0..req_result.len() {
            assert_eq!(req_result[i].r#type, result[i].r#type);
            assert_eq!(req_result[i].value, result[i].value);
        }
    }
}

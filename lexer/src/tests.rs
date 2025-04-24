#[cfg(test)]
mod tests {

    use crate::{
        lexer::*,
        types::{Datatype, Delimiter, Keyword, Operator, Types::*},
    };

    #[test]
    fn check_lexer_1() {
        let contents = r#"
let u32 a = 1
let u32! b = 2
"#;

        let tokens = Lexer::new(contents).tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::new(KEYWORD(Keyword::LET), None, 0, 0),
                Token::new(DATATYPE(Datatype::U32), None, 0, 0),
                Token::new(IDENTIFIER, Some("a".to_string()), 0, 0),
                Token::new(OPERATOR(Operator::ASSIGN), None, 0, 0),
                Token::new(NUMBER, Some("1".to_string()), 0, 0),
                Token::new(NL, None, 0, 0),
                Token::new(KEYWORD(Keyword::LET), None, 0, 0),
                Token::new(DATATYPE(Datatype::U32), None, 0, 0),
                Token::new(OPERATOR(Operator::NOT), None, 0, 0),
                Token::new(IDENTIFIER, Some("b".to_string()), 0, 0),
                Token::new(OPERATOR(Operator::ASSIGN), None, 0, 0),
                Token::new(NUMBER, Some("2".to_string()), 0, 0),
                Token::new(NL, None, 0, 0),
                Token::new(EOF, None, 0, 0),
            ]
        );
    }

    #[test]
    fn check_lexer_2() {
        let contents = r#"
        let string a = "Hello World"
        std::io::println(a)
        "#;
        let tokens = Lexer::new(contents).tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::new(KEYWORD(Keyword::LET), None, 0, 0),
                Token::new(DATATYPE(Datatype::STRING(0)), None, 0, 0),
                Token::new(IDENTIFIER, Some("a".to_string()), 0, 0),
                Token::new(OPERATOR(Operator::ASSIGN), None, 0, 0),
                Token::new(
                    DATATYPE(Datatype::STRING(11)),
                    Some("Hello World".to_string()),
                    0,
                    0
                ),
                Token::new(NL, None, 0, 0),
                Token::new(IDENTIFIER, Some("std".to_string()), 0, 0),
                Token::new(OPERATOR(Operator::PATH), None, 0, 0),
                Token::new(IDENTIFIER, Some("io".to_string()), 0, 0),
                Token::new(OPERATOR(Operator::PATH), None, 0, 0),
                Token::new(IDENTIFIER_FUNC, Some("println".to_string()), 0, 0),
                Token::new(DELIMITER(Delimiter::LPAREN), None, 0, 0),
                Token::new(IDENTIFIER, Some("a".to_string()), 0, 0),
                Token::new(DELIMITER(Delimiter::RPAREN), None, 0, 0),
                Token::new(NL, None, 0, 0),
                Token::new(EOF, None, 0, 0),
            ]
        );
    }

    #[test]
    fn check_lexer_3() {
        let contents = r#"
            struct A {
                a u32,
                b u32,
            }
            "#;
        let tokens = Lexer::new(contents).tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::new(KEYWORD(Keyword::STRUCT), None, 0, 0),
                Token::new(IDENTIFIER, Some("A".to_string()), 0, 0),
                Token::new(DELIMITER(Delimiter::LBRACE), None, 0, 0),
                Token::new(IDENTIFIER, Some("a".to_string()), 0, 0),
                Token::new(DATATYPE(Datatype::U32), None, 0, 0),
                Token::new(DELIMITER(Delimiter::COMMA), None, 0, 0),
                Token::new(IDENTIFIER, Some("b".to_string()), 0, 0),
                Token::new(DATATYPE(Datatype::U32), None, 0, 0),
                Token::new(DELIMITER(Delimiter::COMMA), None, 0, 0),
                Token::new(DELIMITER(Delimiter::RBRACE), None, 0, 0),
                Token::new(NL, None, 0, 0),
                Token::new(EOF, None, 0, 0),
            ]
        );
    }
}

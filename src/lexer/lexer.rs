use core::str;

use crate::errors;

use super::types::{Types, DATATYPE, DELIMITER, KEYWORD, OPERATOR};

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub r#type: Types,
    pub value: Option<String>,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(r#type: Types, value: Option<String>, line: usize, column: usize) -> Self {
        Self {
            r#type,
            value,
            line,
            column,
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            r#type: Types::NL,
            value: None,
            line: 0,
            column: 0,
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    content: String,
    index: usize,
    prev_index: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            index: 0,
            prev_index: 0,
            line: 0,
            column: 0,
        }
    }

    fn next_token(&mut self) -> usize {
        self.index += 1;
        self.index
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while self.index < self.content.len() {
            let char = self.content.as_bytes()[self.index];

            let token = self.check_char(char);

            if let Some(token) = token {
                tokens.push(token);
            }

            self.next_token();
        }

        tokens.push(Token::new(Types::EOF, None, self.line, self.column));
        tokens
    }

    fn check_char(&mut self, char: u8) -> Option<Token> {
        self.column += self.index - self.prev_index;
        self.prev_index = self.index;

        if 48 <= char && char <= 57 {
            return Some(self.check_number());
        } else if 65 <= char && char <= 90 || 97 <= char && char <= 122 {
            return Some(self.check_identifier());
        } else if char == 32 || char == 9 {
            return None;
        } else if char == 34 || char == 39 {
            return Some(self.check_string());
        }

        return Some(Token::new(
            match str::from_utf8(&[char]).unwrap() {
                "+" => Types::OPERATOR(OPERATOR::PLUS),
                "-" => Types::OPERATOR(OPERATOR::MINUS),
                "*" => Types::OPERATOR(OPERATOR::MULTIPLY),
                "," => Types::DELIMITER(DELIMITER::COMMA),
                ";" => Types::DELIMITER(DELIMITER::SEMICOLON),
                "(" => Types::DELIMITER(DELIMITER::LPAREN),
                ")" => Types::DELIMITER(DELIMITER::RPAREN),
                "{" => Types::DELIMITER(DELIMITER::LBRACE),
                "}" => Types::DELIMITER(DELIMITER::RBRACE),
                "[" => Types::DELIMITER(DELIMITER::LBRACKET),
                "]" => Types::DELIMITER(DELIMITER::RBRACKET),
                "." => Types::OPERATOR(OPERATOR::DOT),
                "/" => return self.skip_comment(),
                "\n" => {
                    self.line += 1;
                    self.column = 0;
                    Types::NL
                }
                "=" | "<" | ">" | "!" => self.check_operator(),
                _ => errors::lexer_error(char, "invalid character", self.line, self.column),
            },
            None,
            self.line,
            self.column,
        ));
    }

    fn skip_comment(&mut self) -> Option<Token> {
        let second_char = self.content.as_bytes()[self.index + 1];

        if second_char == 47 {
            while self.content.as_bytes()[self.index + 1] != 10 {
                self.index += 1;
            }
            self.index += 2;
            return None;
        }

        return Some(Token::new(
            Types::OPERATOR(OPERATOR::DIVIDE),
            None,
            self.line,
            self.column,
        ));
    }

    fn check_operator(&mut self) -> Types {
        let first_char = self.content.as_bytes()[self.index];
        let second_char = self.content.as_bytes()[self.index + 1];

        self.index += 2;

        match (first_char, second_char) {
            (61, 61) => return Types::OPERATOR(OPERATOR::EQUAL),
            (33, 61) => return Types::OPERATOR(OPERATOR::NOT_EQUAL),
            (60, 61) => return Types::OPERATOR(OPERATOR::LESSER_EQUAL),
            (62, 61) => return Types::OPERATOR(OPERATOR::GREATER_EQUAL),
            _ => self.index -= 1,
        };

        match first_char {
            33 => Types::OPERATOR(OPERATOR::NOT),
            61 => Types::OPERATOR(OPERATOR::ASSIGN),
            60 => Types::OPERATOR(OPERATOR::LESSER),
            62 => Types::OPERATOR(OPERATOR::GREATER),
            _ => errors::lexer_error(first_char, "invalid token", self.line, self.column),
        }
    }

    fn check_identifier(&mut self) -> Token {
        let start = self.index;
        let mut end = start;

        let mut char = self.content.as_bytes()[self.index];

        while 48 <= char && char <= 57 || 65 <= char && char <= 90 || 97 <= char && char <= 122 {
            end += 1;
            char = self.content.as_bytes()[end];
        }

        let result = self.content[start..end].to_string();

        self.index = end - 1;

        match result.as_str() {
            "func" => Token::new(
                Types::KEYWORD(KEYWORD::FUNCTION),
                None,
                self.line,
                self.column,
            ),
            "let" => Token::new(Types::KEYWORD(KEYWORD::LET), None, self.line, self.column),
            "return" => Token::new(
                Types::KEYWORD(KEYWORD::RETURN),
                None,
                self.line,
                self.column,
            ),
            "if" => Token::new(Types::KEYWORD(KEYWORD::IF), None, self.line, self.column),
            "else" => Token::new(Types::KEYWORD(KEYWORD::ELSE), None, self.line, self.column),
            "loop" => Token::new(Types::KEYWORD(KEYWORD::LOOP), None, self.line, self.column),
            "range" => Token::new(Types::KEYWORD(KEYWORD::RANGE), None, self.line, self.column),
            "u8" => Token::new(Types::DATATYPE(DATATYPE::U8), None, self.line, self.column),
            "u32" => Token::new(Types::DATATYPE(DATATYPE::U32), None, self.line, self.column),
            "u64" => Token::new(Types::DATATYPE(DATATYPE::U64), None, self.line, self.column),

            "i8" => Token::new(Types::DATATYPE(DATATYPE::I8), None, self.line, self.column),
            "i16" => Token::new(Types::DATATYPE(DATATYPE::I16), None, self.line, self.column),
            "i32" => Token::new(Types::DATATYPE(DATATYPE::I32), None, self.line, self.column),
            "i64" => Token::new(Types::DATATYPE(DATATYPE::I64), None, self.line, self.column),
            "f32" => Token::new(Types::DATATYPE(DATATYPE::F32), None, self.line, self.column),
            "f64" => Token::new(Types::DATATYPE(DATATYPE::F64), None, self.line, self.column),
            "bool" => Token::new(
                Types::DATATYPE(DATATYPE::BOOL),
                None,
                self.line,
                self.column,
            ),
            "true" => Token::new(Types::BOOL, Some("1".to_string()), self.line, self.column),
            "false" => Token::new(Types::BOOL, Some("0".to_string()), self.line, self.column),
            "string" => Token::new(
                Types::DATATYPE(DATATYPE::STRING(0)),
                None,
                self.line,
                self.column,
            ),
            _ => {
                if self.content.as_bytes()[self.index + 1] == 40 {
                    Token::new(Types::IDENTIFIER_FUNC, Some(result), self.line, self.column)
                } else {
                    Token::new(Types::IDENTIFIER, Some(result), self.line, self.column)
                }
            }
        }
    }

    fn check_number(&mut self) -> Token {
        let start = self.index;
        let mut end = start;

        let mut char = self.content.as_bytes()[self.index];

        while 48 <= char && char <= 57 {
            end += 1;
            char = self.content.as_bytes()[end];
        }

        let result = self.content[start..end].to_string();

        self.index = end - 1;

        return Token::new(Types::NUMBER, Some(result), self.line, self.column);
    }

    fn check_string(&mut self) -> Token {
        let start = self.index + 1;
        let mut end = start;

        let mut char = self.content.as_bytes()[start];

        while 34 != char && char != 39 {
            end += 1;
            char = self.content.as_bytes()[end];
        }

        let result = self.content[start..end].to_string();

        self.index = end;

        return Token::new(
            Types::DATATYPE(DATATYPE::STRING(result.len())),
            Some(result),
            self.line,
            self.column,
        );
    }
}

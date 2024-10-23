use core::str;

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

            let token = Self::check_char(self, char);

            if let Some(token) = token {
                tokens.push(token);
            }

            Self::next_token(self);
        }

        tokens.push(Token::new(Types::EOF, None, self.line, self.column));
        tokens
    }

    fn check_char(&mut self, char: u8) -> Option<Token> {
        self.column += self.index - self.prev_index;
        self.prev_index = self.index;
        match str::from_utf8(&[char]).unwrap() {
            " " | "\t" => {
                return None;
            }
            "\n" => {
                self.line += 1;
                self.column = 0;
                return Some(Token::new(Types::NL, None, self.line, self.column));
            }
            "+" => {
                return Some(Token::new(
                    Types::OPERATOR(OPERATOR::PLUS),
                    None,
                    self.line,
                    self.column,
                ))
            }
            "-" => {
                return Some(Token::new(
                    Types::OPERATOR(OPERATOR::MINUS),
                    None,
                    self.line,
                    self.column,
                ))
            }
            "*" => {
                return Some(Token::new(
                    Types::OPERATOR(OPERATOR::MULTIPLY),
                    None,
                    self.line,
                    self.column,
                ))
            }
            "/" => return self.skip_comment(),
            "," => {
                return Some(Token::new(
                    Types::DELIMITER(DELIMITER::COMMA),
                    None,
                    self.line,
                    self.column,
                ))
            }
            ";" => {
                return Some(Token::new(
                    Types::DELIMITER(DELIMITER::SEMICOLON),
                    None,
                    self.line,
                    self.column,
                ))
            }
            "(" => {
                return Some(Token::new(
                    Types::DELIMITER(DELIMITER::LPAREN),
                    None,
                    self.line,
                    self.column,
                ))
            }
            ")" => {
                return Some(Token::new(
                    Types::DELIMITER(DELIMITER::RPAREN),
                    None,
                    self.line,
                    self.column,
                ))
            }
            "{" => {
                return Some(Token::new(
                    Types::DELIMITER(DELIMITER::LBRACE),
                    None,
                    self.line,
                    self.column,
                ))
            }
            "}" => {
                return Some(Token::new(
                    Types::DELIMITER(DELIMITER::RBRACE),
                    None,
                    self.line,
                    self.column,
                ))
            }
            "[" => {
                return Some(Token::new(
                    Types::DELIMITER(DELIMITER::LBRACKET),
                    None,
                    self.line,
                    self.column,
                ))
            }
            "]" => {
                return Some(Token::new(
                    Types::DELIMITER(DELIMITER::RBRACKET),
                    None,
                    self.line,
                    self.column,
                ))
            }
            "=" | "<" | ">" | "!" => return Some(self.check_operator()),
            "\"" | "'" => return Some(Self::check_string(self)),
            _ => {
                if 48 <= char && char <= 57 {
                    return Some(Self::check_number(self));
                } else if 65 <= char && char <= 90 || 97 <= char && char <= 122 {
                    return Some(Self::check_identifier(self));
                } else {
                    panic!(
                        "Invalid character: {}, {}",
                        char,
                        str::from_utf8(&[char]).unwrap()
                    );
                }
            }
        }
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

    fn check_operator(&mut self) -> Token {
        let first_char = self.content.as_bytes()[self.index];
        let second_char = self.content.as_bytes()[self.index + 1];

        self.index += 2;

        match (first_char, second_char) {
            (61, 61) => {
                return Token::new(
                    Types::OPERATOR(OPERATOR::EQUAL),
                    None,
                    self.line,
                    self.column,
                )
            }
            (33, 61) => {
                return Token::new(
                    Types::OPERATOR(OPERATOR::NOT_EQUAL),
                    None,
                    self.line,
                    self.column,
                )
            }
            (60, 61) => {
                return Token::new(
                    Types::OPERATOR(OPERATOR::LESSER_EQUAL),
                    None,
                    self.line,
                    self.column,
                )
            }
            (62, 61) => {
                return Token::new(
                    Types::OPERATOR(OPERATOR::GREATER_EQUAL),
                    None,
                    self.line,
                    self.column,
                )
            }
            _ => self.index -= 1,
        }

        match first_char {
            33 => Token::new(Types::OPERATOR(OPERATOR::NOT), None, self.line, self.column),
            61 => Token::new(
                Types::OPERATOR(OPERATOR::ASSIGN),
                None,
                self.line,
                self.column,
            ),
            60 => Token::new(
                Types::OPERATOR(OPERATOR::LESSER),
                None,
                self.line,
                self.column,
            ),
            62 => Token::new(
                Types::OPERATOR(OPERATOR::GREATER),
                None,
                self.line,
                self.column,
            ),
            _ => panic!("Unexpected token"),
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
            "u16" => Token::new(Types::DATATYPE(DATATYPE::U16), None, self.line, self.column),
            "u32" => Token::new(Types::DATATYPE(DATATYPE::U32), None, self.line, self.column),
            "u64" => Token::new(Types::DATATYPE(DATATYPE::U64), None, self.line, self.column),

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
                Types::DATATYPE(DATATYPE::STRING),
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

        let mut char = self.content.as_bytes()[self.index];

        while 34 != char && char != 39 {
            end += 1;
            char = self.content.as_bytes()[end];
        }

        let result = self.content[start..end].to_string();

        self.index = end - 1;

        return Token::new(
            Types::DATATYPE(DATATYPE::STRING),
            Some(result),
            self.line,
            self.column,
        );
    }
}

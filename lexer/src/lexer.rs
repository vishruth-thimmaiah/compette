use core::str;

use crate::lexer_error;

use super::types::{Datatype, Delimiter, Keyword, Operator, Types};

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

        if b'0' <= char && char <= b'9' {
            return Some(self.check_number());
        } else if b'A' <= char && char <= b'Z' || b'a' <= char && char <= b'z' || char == b'_' {
            return Some(self.check_identifier());
        } else if char == b' ' || char == b'\t' {
            return None;
        } else if char == b'"' || char == b'\'' {
            return Some(self.check_string());
        }

        return Some(Token::new(
            match char {
                b'+' => Types::OPERATOR(Operator::PLUS),
                b'*' => Types::OPERATOR(Operator::MULTIPLY),
                b',' => Types::DELIMITER(Delimiter::COMMA),
                b';' => Types::DELIMITER(Delimiter::SEMICOLON),
                b'(' => Types::DELIMITER(Delimiter::LPAREN),
                b')' => Types::DELIMITER(Delimiter::RPAREN),
                b'{' => Types::DELIMITER(Delimiter::LBRACE),
                b'}' => Types::DELIMITER(Delimiter::RBRACE),
                b'[' => Types::DELIMITER(Delimiter::LBRACKET),
                b']' => Types::DELIMITER(Delimiter::RBRACKET),
                b'.' => Types::OPERATOR(Operator::DOT),
                b':' => Types::OPERATOR(Operator::COLON),
                b'/' => return self.skip_comment(),
                b'\n' => {
                    self.line += 1;
                    self.column = 0;
                    Types::NL
                }
                b'=' | b'<' | b'>' | b'!' | b'-' => self.check_multi_char_type(),
                _ => lexer_error(char, "invalid character", self.line, self.column),
            },
            None,
            self.line,
            self.column,
        ));
    }

    fn skip_comment(&mut self) -> Option<Token> {
        let second_char = self.content.as_bytes()[self.index + 1];

        if second_char == b'/' {
            while self.content.as_bytes()[self.index + 1] != b'\n' {
                self.index += 1;
            }
            self.index += 2;
            return None;
        }

        return Some(Token::new(
            Types::OPERATOR(Operator::DIVIDE),
            None,
            self.line,
            self.column,
        ));
    }

    fn check_multi_char_type(&mut self) -> Types {
        let first_char = self.content.as_bytes()[self.index];
        let second_char = self.content.as_bytes()[self.index + 1];

        self.index += 2;

        match (first_char, second_char) {
            (b'=', b'=') => return Types::OPERATOR(Operator::EQUAL),
            (b'!', b'=') => return Types::OPERATOR(Operator::NOT_EQUAL),
            (b'<', b'=') => return Types::OPERATOR(Operator::LESSER_EQUAL),
            (b'>', b'=') => return Types::OPERATOR(Operator::GREATER_EQUAL),
            (b'-', b'>') => return Types::OPERATOR(Operator::CAST),
            _ => self.index -= 1,
        };

        match first_char {
            b'!' => Types::OPERATOR(Operator::NOT),
            b'=' => Types::OPERATOR(Operator::ASSIGN),
            b'<' => Types::OPERATOR(Operator::LESSER),
            b'>' => Types::OPERATOR(Operator::GREATER),
            b'-' => Types::OPERATOR(Operator::MINUS),
            _ => lexer_error(first_char, "invalid token", self.line, self.column),
        }
    }

    fn check_identifier(&mut self) -> Token {
        let start = self.index;
        let mut end = start;

        let mut char = self.content.as_bytes()[self.index];

        while b'0' <= char && char <= b'9'
            || b'a' <= char && char <= b'z'
            || b'A' <= char && char <= b'Z'
            || char == b'_'
        {
            end += 1;
            char = self.content.as_bytes()[end];
        }

        let result = self.content[start..end].to_string();

        self.index = end - 1;

        let (token_type, token_value) = match result.as_str() {
            "struct" => (Types::KEYWORD(Keyword::STRUCT), None),
            "func" => (Types::KEYWORD(Keyword::FUNCTION), None),
            "import" => (Types::KEYWORD(Keyword::IMPORT), None),
            "let" => (Types::KEYWORD(Keyword::LET), None),
            "return" => (Types::KEYWORD(Keyword::RETURN), None),
            "if" => (Types::KEYWORD(Keyword::IF), None),
            "else" => (Types::KEYWORD(Keyword::ELSE), None),
            "loop" => (Types::KEYWORD(Keyword::LOOP), None),
            "range" => (Types::KEYWORD(Keyword::RANGE), None),
            "break" => (Types::KEYWORD(Keyword::BREAK), None),
            "u8" => (Types::DATATYPE(Datatype::U8), None),
            "u16" => (Types::DATATYPE(Datatype::U16), None),
            "u32" => (Types::DATATYPE(Datatype::U32), None),
            "u64" => (Types::DATATYPE(Datatype::U64), None),
            "i8" => (Types::DATATYPE(Datatype::I8), None),
            "i16" => (Types::DATATYPE(Datatype::I16), None),
            "i32" => (Types::DATATYPE(Datatype::I32), None),
            "i64" => (Types::DATATYPE(Datatype::I64), None),
            "f32" => (Types::DATATYPE(Datatype::F32), None),
            "f64" => (Types::DATATYPE(Datatype::F64), None),
            "bool" => (Types::DATATYPE(Datatype::BOOL), None),
            "true" => (Types::BOOL, Some("1".to_string())),
            "false" => (Types::BOOL, Some("0".to_string())),
            "string" => (Types::DATATYPE(Datatype::STRING(0)), None),
            _ => {
                if self.content.as_bytes()[self.index + 1] == b':' {
                    self.index += 1;
                    (Types::IMPORT_CALL, Some(result))
                } else if self.content.as_bytes()[self.index + 1] == b'(' {
                    (Types::IDENTIFIER_FUNC, Some(result))
                } else {
                    (Types::IDENTIFIER, Some(result))
                }
            }
        };

        Token::new(token_type, token_value, self.line, self.column)
    }

    fn check_number(&mut self) -> Token {
        let start = self.index;
        let mut end = start;

        let mut char = self.content.as_bytes()[self.index];

        while (b'0' <= char && char <= b'9') || char == b'.' {
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
            Types::DATATYPE(Datatype::STRING(result.len())),
            Some(result),
            self.line,
            self.column,
        );
    }
}

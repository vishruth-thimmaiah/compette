use core::str;

use crate::lexer_error;

use super::types::{Datatype, Delimiter, Keyword, Operator, Types};

#[derive(Debug, Clone)]
pub struct Token {
    pub r#type: Types,
    pub value: Option<String>,
    pub line: usize,
    pub column: usize,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.value == other.value
    }
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
pub struct Lexer<'a> {
    content: &'a [u8],
    index: usize,
    prev_index: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            content: content.as_bytes(),
            index: 0,
            prev_index: 0,
            line: 0,
            column: 0,
        }
    }

    fn next_byte(&mut self) -> Option<u8> {
        if self.index < self.content.len() {
            let token = self.content[self.index];
            self.index += 1;
            return Some(token);
        } else {
            None
        }
    }

    fn peek_byte(&self) -> Option<u8> {
        if self.index < self.content.len() {
            Some(self.content[self.index])
        } else {
            None
        }
    }

    fn current_byte(&self) -> u8 {
        self.content[self.index - 1]
    }

    fn previous_byte(&mut self) -> u8 {
        self.index -= 1;
        self.content[self.index - 1]
    }

    fn get_range(&self, start: usize, end: usize) -> String {
        let a = &self.content[start..end];
        String::from_utf8(a.to_vec()).unwrap()
    }

    fn get_relative_range(&self, size: usize) -> String {
        let a = &self.content[self.index - size..self.index];
        String::from_utf8(a.to_vec()).unwrap()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(char) = self.next_byte() {
            self.column += self.index - self.prev_index;
            self.prev_index = self.index;

            let token = match char {
                b'0'..=b'9' => self.tokenize_number(),
                b'A'..=b'Z' | b'a'..=b'z' | b'_' => self.tokenize_identifier(),
                b'"' | b'\'' => self.tokenize_string(),
                b' ' | b'\t' => None,
                _ => self.tokenize_symbols(char, &mut tokens),
            };

            if let Some(token) = token {
                tokens.push(token);
            }
        }
        tokens.push(Token::new(Types::EOF, None, self.line, self.column));
        tokens
    }

    fn tokenize_symbols(&mut self, char: u8, tokens: &mut Vec<Token>) -> Option<Token> {
        return Some(Token::new(
            match char {
                b'+' => Types::OPERATOR(Operator::PLUS),
                b'*' => Types::OPERATOR(Operator::MULTIPLY),
                b',' => Types::DELIMITER(Delimiter::COMMA),
                b';' => Types::DELIMITER(Delimiter::SEMICOLON),
                b'(' => Types::DELIMITER(Delimiter::LPAREN),
                b')' => Types::DELIMITER(Delimiter::RPAREN),
                b'[' => Types::DELIMITER(Delimiter::LBRACKET),
                b']' => Types::DELIMITER(Delimiter::RBRACKET),
                b'.' => Types::OPERATOR(Operator::DOT),
                b'{' => Types::DELIMITER(Delimiter::LBRACE),
                b'^' => Types::OPERATOR(Operator::BITWISE_XOR),
                b'&' => Types::OPERATOR(Operator::BITWISE_AND),
                b'|' => Types::OPERATOR(Operator::BITWISE_OR),
                b'}' => {
                    self.pop_nl(tokens);
                    Types::DELIMITER(Delimiter::RBRACE)
                }
                b'/' => return self.skip_comment(),
                b'\n' => self.tokenize_nl(tokens)?,
                b'=' | b'<' | b'>' | b'!' | b'-' | b':' => self.check_multi_char_type()?,
                _ => lexer_error(char, "invalid character", self.line, self.column),
            },
            None,
            self.line,
            self.column,
        ));
    }

    fn skip_comment(&mut self) -> Option<Token> {
        let second_char = self.content[self.index + 1];

        if second_char == b'/' {
            while self.peek_byte()? != b'\n' {
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

    fn check_multi_char_type(&mut self) -> Option<Types> {
        let first_char = self.current_byte();
        let second_char = self.next_byte()?;

        match (first_char, second_char) {
            (b'=', b'=') => return Some(Types::OPERATOR(Operator::EQUAL)),
            (b'!', b'=') => return Some(Types::OPERATOR(Operator::NOT_EQUAL)),
            (b'<', b'=') => return Some(Types::OPERATOR(Operator::LESSER_EQUAL)),
            (b'>', b'=') => return Some(Types::OPERATOR(Operator::GREATER_EQUAL)),
            (b'-', b'>') => return Some(Types::OPERATOR(Operator::CAST)),
            (b':', b':') => return Some(Types::OPERATOR(Operator::PATH)),
            (b'>', b'>') => return Some(Types::OPERATOR(Operator::RSHIFT)),
            (b'<', b'<') => return Some(Types::OPERATOR(Operator::LSHIFT)),
            _ => self.previous_byte(),
        };

        match first_char {
            b'!' => Some(Types::OPERATOR(Operator::NOT)),
            b'=' => Some(Types::OPERATOR(Operator::ASSIGN)),
            b'<' => Some(Types::OPERATOR(Operator::LESSER)),
            b'>' => Some(Types::OPERATOR(Operator::GREATER)),
            b'-' => Some(Types::OPERATOR(Operator::MINUS)),
            b':' => Some(Types::OPERATOR(Operator::COLON)),
            _ => lexer_error(first_char, "invalid token", self.line, self.column),
        }
    }

    fn tokenize_identifier(&mut self) -> Option<Token> {
        let mut size = 0;

        let mut char = self.current_byte();

        while b'0' <= char && char <= b'9'
            || b'a' <= char && char <= b'z'
            || b'A' <= char && char <= b'Z'
            || char == b'_'
        {
            size += 1;
            char = self.next_byte()?;
        }
        self.previous_byte();

        let result = self.get_relative_range(size);

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
                if self.peek_byte()? == b'(' {
                    (Types::IDENTIFIER_FUNC, Some(result))
                } else {
                    (Types::IDENTIFIER, Some(result))
                }
            }
        };

        Some(Token::new(token_type, token_value, self.line, self.column))
    }

    fn tokenize_number(&mut self) -> Option<Token> {
        let mut size = 0;

        let mut char = self.current_byte();

        while (b'0' <= char && char <= b'9') || char == b'.' {
            size += 1;
            char = self.next_byte()?;
        }
        self.previous_byte();

        let result = self.get_relative_range(size);

        return Some(Token::new(
            Types::NUMBER,
            Some(result),
            self.line,
            self.column,
        ));
    }

    fn tokenize_string(&mut self) -> Option<Token> {
        let start = self.index;
        let mut end = start;

        let mut char = self.next_byte()?;

        while b'\'' != char && char != b'"' {
            end += 1;
            char = self.next_byte()?;
        }
        let result = self.get_range(start, end);

        return Some(Token::new(
            Types::DATATYPE(Datatype::STRING(result.len())),
            Some(result),
            self.line,
            self.column,
        ));
    }

    fn tokenize_nl(&mut self, token: &Vec<Token>) -> Option<Types> {
        self.column = 0;
        self.line += 1;
        match token.last()?.r#type {
            Types::NL => None,
            Types::DELIMITER(Delimiter::COMMA) | Types::DELIMITER(Delimiter::LBRACE) => None,
            _ => Some(Types::NL),
        }
    }

    fn pop_nl(&mut self, tokens: &mut Vec<Token>) {
        while let Some(token) = tokens.last() {
            if token.r#type == Types::NL {
                tokens.pop();
            } else {
                return;
            }
        }
    }
}

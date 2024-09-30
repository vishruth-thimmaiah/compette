use core::str;

use super::types::Types;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub r#type: Types,
    pub value: Option<String>,
}

impl Token {
    pub fn new(r#type: Types, value: Option<String>) -> Self {
        Self { r#type, value }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            r#type: Types::NL,
            value: None,
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    content: String,
    index: usize,
}

impl Lexer {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            index: 0,
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

        tokens.push(Token::new(Types::EOF, None));
        tokens
    }

    fn check_char(&mut self, char: u8) -> Option<Token> {
        match str::from_utf8(&[char]).unwrap() {
            " " | "\t" => {
                return None;
            }
            "\n" => return Some(Token::new(Types::NL, None)),
            "=" => return Some(Token::new(Types::ASSIGN, None)),
            "+" => return Some(Token::new(Types::PLUS, None)),
            "-" => return Some(Token::new(Types::MINUS, None)),
            "*" => return Some(Token::new(Types::MULTIPLY, None)),
            "/" => return Some(Token::new(Types::DIVIDE, None)),
            "," => return Some(Token::new(Types::COMMA, None)),
            ";" => return Some(Token::new(Types::SEMICOLON, None)),
            "(" => return Some(Token::new(Types::LPAREN, None)),
            ")" => return Some(Token::new(Types::RPAREN, None)),
            "{" => return Some(Token::new(Types::LBRACE, None)),
            "}" => return Some(Token::new(Types::RBRACE, None)),
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

    fn check_identifier(&mut self) -> Token {
        let start = self.index;
        let mut end = start;

        let mut char = self.content.as_bytes()[self.index];

        while 65 <= char && char <= 90 || 97 <= char && char <= 122 {
            end += 1;
            char = self.content.as_bytes()[end];
        }

        let result = self.content[start..end].to_string();

        self.index = end - 1;

        match result.as_str() {
            "func" => Token::new(Types::FUNCTION, None),
            "let" => Token::new(Types::LET, None),
            _ => Token::new(Types::IDENTIFIER, Some(result)),
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

        return Token::new(Types::NUMBER, Some(result));
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

        return Token::new(Types::STRING, Some(result));
    }
}

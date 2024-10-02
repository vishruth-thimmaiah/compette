use core::str;

use super::types::Types;

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
            "+" => return Some(Token::new(Types::PLUS, None, self.line, self.column)),
            "-" => return Some(Token::new(Types::MINUS, None, self.line, self.column)),
            "*" => return Some(Token::new(Types::MULTIPLY, None, self.line, self.column)),
            "/" => return Some(Token::new(Types::DIVIDE, None, self.line, self.column)),
            "," => return Some(Token::new(Types::COMMA, None, self.line, self.column)),
            ";" => return Some(Token::new(Types::SEMICOLON, None, self.line, self.column)),
            "(" => return Some(Token::new(Types::LPAREN, None, self.line, self.column)),
            ")" => return Some(Token::new(Types::RPAREN, None, self.line, self.column)),
            "{" => return Some(Token::new(Types::LBRACE, None, self.line, self.column)),
            "}" => return Some(Token::new(Types::RBRACE, None, self.line, self.column)),
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

    fn check_operator(&mut self) -> Token {
        let first_char = self.content.as_bytes()[self.index];
        let second_char = self.content.as_bytes()[self.index + 1];

        self.index += 2;

        match (first_char, second_char) {
            (61, 61) => return Token::new(Types::EQUAL, None, self.line, self.column),
            (33, 61) => return Token::new(Types::NOT_EQUAL, None, self.line, self.column),
            (60, 61) => return Token::new(Types::LESSER_EQUAL, None, self.line, self.column),
            (62, 61) => return Token::new(Types::GREATER_EQUAL, None, self.line, self.column),
            _ => self.index -= 1,
        }

        match first_char {
            61 => Token::new(Types::ASSIGN, None, self.line, self.column),
            60 => Token::new(Types::LESSER, None, self.line, self.column),
            62 => Token::new(Types::GREATER, None, self.line, self.column),
            _ => panic!("Unexpected token"),
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
            "func" => Token::new(Types::FUNCTION, None, self.line, self.column),
            "let" => Token::new(Types::LET, None, self.line, self.column),
            "return" => Token::new(Types::RETURN, None, self.line, self.column),
            "if" => Token::new(Types::IF, None, self.line, self.column),
            "else" => Token::new(Types::ELSE, None, self.line, self.column),
            _ => Token::new(Types::IDENTIFIER, Some(result), self.line, self.column),
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

        return Token::new(Types::STRING, Some(result), self.line, self.column);
    }
}

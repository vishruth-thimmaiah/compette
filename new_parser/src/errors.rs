use std::{error::Error, fmt::Display};

use lexer::{lexer::Token, types::Types};

#[derive(Debug)]
pub struct ParserError {
    pub(crate) msg: String,
    pub(crate) line: usize,
    pub(crate) column: usize,
}

impl Error for ParserError {}

impl Default for ParserError {
    fn default() -> Self {
        Self {
            msg: "Unknown error while parsing".to_string(),
            line: 0,
            column: 0,
        }
    }
}

impl ParserError {
    pub(crate) fn new(msg: &str, token: Token) -> Self {
        Self {
            msg: msg.to_string(),
            line: token.line,
            column: token.column,
        }
    }

    pub(crate) fn expected_token_err(token: Token, expected: Types) -> Self {
        Self::new(
            &format!("Expected token {:?}, got {:?}", expected, token.r#type),
            token,
        )
    }

    pub(crate) fn unexpected_token_err(token: Token) -> Self {
        Self::new(&format!("Unexpected token {:?}", token.r#type), token)
    }

    pub(crate) fn unexpected_eof(token: Option<Token>) -> Self {
        if let Some(token) = token {
            Self::new(
                &format!("Expected token {:?}, got eof", token.r#type),
                token,
            )
        } else {
            Self::new("Expected token, got eof", Token::default())
        }
    }

    pub(crate) fn unimplemented(token: Token) -> Self {
        Self::new(&format!("Unimplemented token {:?}", token.r#type), token)
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub(crate) type Result<T> = std::result::Result<T, ParserError>;

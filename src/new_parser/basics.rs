use crate::lexer::types::{Types, DATATYPE};

use super::{Parser, ParserError, Result};

impl Parser {
    pub fn parse_datatype(&mut self) -> Result<DATATYPE> {
        let token = self.next().ok_or(ParserError::default())?;
        let Types::DATATYPE(dt) = token.r#type else {
            return Err(ParserError::default());
        };
        Ok(dt)
    }
}

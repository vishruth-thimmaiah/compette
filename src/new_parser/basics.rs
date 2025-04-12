use crate::lexer::types::{Types, DATATYPE};

use super::{Parser, ParserError};

impl Parser {
    pub fn parse_datatype(&mut self) -> Result<DATATYPE, ParserError> {
        let token = self.next().ok_or(ParserError)?;
        let Types::DATATYPE(dt) = token.r#type else {
            return Err(ParserError);
        };
        Ok(dt)
    }
}

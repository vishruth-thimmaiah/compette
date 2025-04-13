use lexer::types::{Datatype, Types};

use super::{Parser, ParserError, Result};

impl Parser {
    pub fn parse_datatype(&mut self) -> Result<Datatype> {
        let token = self.next().ok_or(ParserError::default())?;
        let Types::DATATYPE(dt) = token.r#type else {
            return Err(ParserError::default());
        };
        Ok(dt)
    }
}

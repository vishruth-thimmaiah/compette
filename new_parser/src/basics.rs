use lexer::types::{Datatype, Types};

use super::{Parser, ParserError, Result};

impl Parser {
    pub(crate) fn parse_datatype(&mut self) -> Result<Datatype> {
        let token = self.next().ok_or(ParserError::default())?;
        let dt = if let Types::DATATYPE(dt) = token.r#type {
            dt
        } else if let Types::IDENTIFIER = token.r#type {
            Datatype::CUSTOM(token.value.unwrap())
        } else {
            return Err(ParserError::default());
        };
        Ok(dt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::lexer::Lexer;

    #[test]
    fn test_parse_datatype() {
        let mut lexer = Lexer::new("u32 ");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse_datatype().unwrap();
        assert_eq!(ast, Datatype::U32);
    }

    #[test]
    fn test_parse_custom_datatype() {
        let mut lexer = Lexer::new("Test ");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse_datatype().unwrap();
        assert_eq!(ast, Datatype::CUSTOM("Test".to_string()));
    }
}

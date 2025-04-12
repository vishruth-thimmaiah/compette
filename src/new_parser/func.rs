use crate::lexer::types::{Types, DATATYPE, DELIMITER};

use super::{
    nodes::{Block, Function},
    Parser, ParserError,
};

impl Parser {
    pub fn parse_function_def(&mut self) -> Result<Function, ParserError> {
        let name = self.next_with_type(Types::IDENTIFIER_FUNC)?;
        self.next_with_type(Types::DELIMITER(DELIMITER::LPAREN))?;
        let mut args: Vec<(String, DATATYPE)> = vec![];
        self.next_with_type(Types::DELIMITER(DELIMITER::RPAREN))?;
        let return_type = self.parse_datatype()?;
        self.next_with_type(Types::DELIMITER(DELIMITER::LBRACE))?;
        self.next_with_type(Types::DELIMITER(DELIMITER::RBRACE))?;

        Ok(Function {
            name: name.value.unwrap(),
            args,
            return_type,
            body: Block { body: vec![] },
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::lexer::Lexer, new_parser::nodes::ASTNodes};

    use super::*;

    #[test]
    fn test_parse_function_def() {
        let mut lexer = Lexer::new("func main() u32 {}");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Function(Function {
                name: "main".to_string(),
                args: vec![],
                return_type: DATATYPE::U32,
                body: Block { body: vec![] },
            })]
        );
    }
}

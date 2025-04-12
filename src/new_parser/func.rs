use crate::lexer::types::{Types, DATATYPE, DELIMITER};

use super::{
    nodes::{Function, Return},
    Parser, Result,
};

impl Parser {
    pub fn parse_function_def(&mut self) -> Result<Function> {
        let name = self.next_with_type(Types::IDENTIFIER_FUNC)?;
        self.next_with_type(Types::DELIMITER(DELIMITER::LPAREN))?;
        let mut args: Vec<(String, DATATYPE)> = vec![];
        self.next_with_type(Types::DELIMITER(DELIMITER::RPAREN))?;
        let return_type = self.parse_datatype()?;
        self.next_with_type(Types::DELIMITER(DELIMITER::LBRACE))?;
        let body = self.parse_function_block()?;

        Ok(Function {
            name: name.value.unwrap(),
            args,
            return_type,
            body,
        })
    }

    pub fn parse_return(&mut self) -> Result<Return> {
        return Ok(Return { value: None });
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::lexer::Lexer,
        new_parser::nodes::{ASTNodes, Block},
    };

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

    #[test]
    fn test_parse_function_def_with_return() {
        let mut lexer = Lexer::new("func main() u32 { return }");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Function(Function {
                name: "main".to_string(),
                args: vec![],
                return_type: DATATYPE::U32,
                body: Block {
                    body: vec![ASTNodes::Return(Return { value: None })]
                },
            }),]
        );
    }
}

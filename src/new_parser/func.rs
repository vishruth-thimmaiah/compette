use crate::lexer::types::{Types, Datatype, Delimiter};

use super::{
    errors::ParserError,
    nodes::{Function, Return},
    Parser, Result,
};

impl Parser {
    pub fn parse_function_def(&mut self) -> Result<Function> {
        let name = self.next_with_type(Types::IDENTIFIER_FUNC)?;
        let args = self.parse_function_args()?;
        let return_type = self.parse_datatype()?;
        let body = self.parse_function_block()?;

        Ok(Function {
            name: name.value.unwrap(),
            args,
            return_type,
            body,
        })
    }

    fn parse_function_args(&mut self) -> Result<Vec<(String, Datatype)>> {
        let mut args: Vec<(String, Datatype)> = vec![];
        self.next_with_type(Types::DELIMITER(Delimiter::LPAREN))?;

        match self.peek().ok_or(ParserError::default())?.r#type {
            Types::DELIMITER(Delimiter::RPAREN) => {
                self.next();
                return Ok(args);
            }
            _ => (),
        }

        loop {
            let var_name = self.next_with_type(Types::IDENTIFIER)?;
            let var_type = self.parse_datatype()?;

            args.push((var_name.value.unwrap(), var_type));

            match self.next().ok_or(ParserError::default())?.r#type {
                Types::DELIMITER(Delimiter::RPAREN) => break,
                Types::DELIMITER(Delimiter::COMMA) => (),
                _ => return Err(ParserError::default()),
            }
        }

        Ok(args)
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
                return_type: Datatype::U32,
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
                return_type: Datatype::U32,
                body: Block {
                    body: vec![ASTNodes::Return(Return { value: None })]
                },
            }),]
        );
    }

    #[test]
    fn test_parse_function_def_with_args() {
        let mut lexer = Lexer::new("func main(a u32, b u32) u32 {}");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Function(Function {
                name: "main".to_string(),
                args: vec![
                    ("a".to_string(), Datatype::U32),
                    ("b".to_string(), Datatype::U32)
                ],
                return_type: Datatype::U32,
                body: Block { body: vec![] },
            })]
        );
    }
}

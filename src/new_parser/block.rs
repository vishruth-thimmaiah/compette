use crate::lexer::types::{Types, Datatype, Delimiter, Keyword};

use super::{
    nodes::{ASTNodes, Block},
    Parser, ParserError, Result,
};

impl Parser {
    pub fn parse_source(&mut self) -> Result<Vec<ASTNodes>> {
        let mut ast = Vec::new();

        let token = self.next().ok_or(ParserError::default())?;

        let object = match token.r#type {
            Types::NL => todo!(),
            Types::KEYWORD(Keyword::FUNCTION) => ASTNodes::Function(self.parse_function_def()?),
            _ => return Err(ParserError::default()),
        };
        ast.push(object);

        Ok(ast)
    }

    pub fn parse_function_block(&mut self) -> Result<Block> {
        self.next_with_type(Types::DELIMITER(Delimiter::LBRACE))?;
        let mut body: Vec<ASTNodes> = vec![];

        while let Some(token) = self.next() {
            let object = match token.r#type {
                Types::NL => todo!(),
                Types::KEYWORD(Keyword::RETURN) => ASTNodes::Return(self.parse_return()?),
                Types::DELIMITER(Delimiter::RBRACE) => break,
                _ => return Err(ParserError::default()),
            };
            body.push(object);
        }

        Ok(Block { body })
    }
}

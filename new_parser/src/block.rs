use lexer::types::{Delimiter, Keyword, Types};

use super::{
    Parser, ParserError, Result,
    nodes::{ASTNodes, Block},
};

impl Parser {
    pub(crate) fn parse_source(&mut self) -> Result<Vec<ASTNodes>> {
        let mut ast = Vec::new();

        while let Some(token) = self.next() {
            let object = match token.r#type {
                Types::NL => continue,
                Types::KEYWORD(Keyword::FUNCTION) => ASTNodes::Function(self.parse_function_def()?),
                Types::EOF => break,
                _ => return Err(ParserError::unimplemented(token)),
            };
            ast.push(object);
        }

        Ok(ast)
    }

    pub(crate) fn parse_function_block(&mut self) -> Result<Block> {
        self.next_with_type(Types::DELIMITER(Delimiter::LBRACE))?;
        let mut body: Vec<ASTNodes> = vec![];

        while let Some(token) = self.next() {
            let object = match token.r#type {
                Types::NL => continue,
                Types::KEYWORD(Keyword::RETURN) => ASTNodes::Return(self.parse_return()?),
                Types::KEYWORD(Keyword::LET) => ASTNodes::LetStmt(self.parse_statement()?),
                Types::DELIMITER(Delimiter::RBRACE) => break,
                _ => return Err(ParserError::unimplemented(token)),
            };
            body.push(object);
        }

        Ok(Block { body })
    }
}

use lexer::types::{Delimiter, Keyword, Operator, Types};

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
                Types::KEYWORD(Keyword::IMPORT) => ASTNodes::ImportDef(self.parse_import_def()?),
                Types::KEYWORD(Keyword::FUNCTION) => ASTNodes::Function(self.parse_function_def()?),
                Types::KEYWORD(Keyword::STRUCT) => ASTNodes::StructDef(self.parse_struct_def()?),
                Types::EOF => break,
                _ => return Err(ParserError::unimplemented(token)),
            };
            ast.push(object);
        }

        Ok(ast)
    }

    pub(crate) fn parse_scoped_block(&mut self) -> Result<Block> {
        self.next_with_type(Types::DELIMITER(Delimiter::LBRACE))?;
        let mut body: Vec<ASTNodes> = vec![];

        while let Some(token) = self.next() {
            let object = match token.r#type {
                Types::NL => continue,
                Types::KEYWORD(Keyword::RETURN) => ASTNodes::Return(self.parse_return()?),
                Types::KEYWORD(Keyword::LET) => ASTNodes::LetStmt(self.parse_statement()?),
                Types::KEYWORD(Keyword::IF) => ASTNodes::Conditional(self.parse_if()?),
                Types::KEYWORD(Keyword::LOOP) => self.parse_loop()?,
                Types::IDENTIFIER_FUNC => ASTNodes::FunctionCall(self.parse_function_call()?),
                Types::IDENTIFIER
                    if self.peek_if_type(Types::OPERATOR(Operator::PATH)).is_some() =>
                {
                    ASTNodes::ImportCall(self.parse_import_call()?)
                }
                Types::IDENTIFIER => ASTNodes::AssignStmt(self.parse_assign_stmt()?),
                Types::DELIMITER(Delimiter::RBRACE) => break,
                _ => return Err(ParserError::unimplemented(token)),
            };
            body.push(object);
        }

        Ok(Block { body })
    }
}

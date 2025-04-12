use crate::lexer::types::{Types, KEYWORD};

use super::{nodes::ASTNodes, Parser, ParserError};

impl Parser {
    pub fn parse_source(&mut self) -> Result<Vec<ASTNodes>, ParserError> {

        let mut ast = Vec::new();

        let token = self.next().ok_or(ParserError)?;

        let object = match token.r#type {
            Types::NL => todo!(),
            Types::KEYWORD(KEYWORD::FUNCTION) => ASTNodes::Function(self.parse_function_def()?),
            _ => return Err(ParserError),
        };
        ast.push(object);

        Ok(ast)
    }
}

// pub fn parse_scope(&mut self) -> Vec<Box<dyn ParserType>> {
//     let mut tokens: Vec<Box<dyn ParserType>> = vec![];
//
//     let mut nested = false;
//
//     while self.position < self.tree.len() {
//         let token_type = self.get_current_token().r#type;
//         match token_type {
//             Types::NL => (),
//             Types::EOF => break,
//             Types::KEYWORD(KEYWORD::STRUCT) => tokens.push(self.parse_def_struct()),
//             Types::KEYWORD(KEYWORD::IMPORT) => tokens.push(self.parse_import()),
//             Types::KEYWORD(KEYWORD::LET) => tokens.push(self.parse_assignment()),
//             Types::KEYWORD(KEYWORD::IF) => tokens.push(self.parse_conditional_if()),
//             Types::KEYWORD(KEYWORD::FUNCTION) => tokens.push(self.parse_function()),
//             Types::KEYWORD(KEYWORD::BREAK) => tokens.push(self.parse_break()),
//             Types::IDENTIFIER => tokens.push(self.parse_identifier_call()),
//             Types::IDENTIFIER_FUNC => tokens.push(self.parse_function_call(None)),
//             Types::IMPORT_CALL => tokens.push(self.parse_import_call()),
//             Types::KEYWORD(KEYWORD::LOOP) => tokens.push(self.parse_loop()),
//             Types::DELIMITER(DELIMITER::LBRACE) => nested = true,
//             // TODO: better function detecting
//             Types::KEYWORD(KEYWORD::RETURN) => {
//                 if nested {
//                     tokens.push(self.parse_return())
//                 }
//             }
//             Types::DELIMITER(DELIMITER::RBRACE) => {
//                 if !nested {
//                     errors::parser_error(self, "Invalid close brace");
//                 }
//                 break;
//             }
//             _ => errors::parser_error(self, "invalid token"),
//         }
//
//         self.position += 1;
//     }
//
//     tokens
// }

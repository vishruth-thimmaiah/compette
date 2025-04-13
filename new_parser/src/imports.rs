use lexer::types::{Operator, Types};

use crate::{Parser, Result, nodes::ImportCall};

impl Parser {
    pub(crate) fn parse_import_call(&mut self) -> Result<ImportCall> {
        let mut path = Vec::new();
        println!("{:?}", self.peek());
        loop {
            let subpath = self.next_with_type(Types::IDENTIFIER)?;
            path.push(subpath.value.unwrap());

            if self.next_if_type(Types::OPERATOR(Operator::PATH)).is_none() {
                break
            }
        }

        return Ok(ImportCall {
            path,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::ASTNodes;

    use super::*;
    use lexer::lexer::Lexer;

    #[test]
    fn test_parse_import_call() {
        let mut lexer = Lexer::new("import std::io ");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::ImportCall(ImportCall {
                path: vec!["std".to_string(), "io".to_string()]
            })]
        );
    }
}

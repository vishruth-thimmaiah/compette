use lexer::types::{Keyword, Operator, Types};

use crate::{
    Parser, Result,
    nodes::{Extern, ImportCall, ImportDef},
};

impl Parser {
    pub(crate) fn parse_import_def(&mut self) -> Result<ImportDef> {
        let mut path = Vec::new();
        loop {
            let subpath = self.next_with_type(Types::IDENTIFIER)?;
            path.push(subpath.value.unwrap());

            if self.next_if_type(Types::OPERATOR(Operator::PATH)).is_none() {
                break;
            }
        }

        return Ok(ImportDef { path });
    }

    pub(crate) fn parse_import_call(&mut self) -> Result<ImportCall> {
        let mut path = Vec::new();
        path.push(self.current_with_type(Types::IDENTIFIER)?.value.unwrap());
        self.next_with_type(Types::OPERATOR(Operator::PATH))?;
        loop {
            if let Some(subpath) = self.next_if_type(Types::IDENTIFIER) {
                path.push(subpath.value.unwrap());
            } else {
                path.push(self.next_with_type(Types::IDENTIFIER_FUNC)?.value.unwrap());
            }

            if self.next_if_type(Types::OPERATOR(Operator::PATH)).is_none() {
                break;
            }
        }

        return Ok(ImportCall {
            path,
            ident: Box::new(self.parse_complex_variable()?),
        });
    }

    pub(crate) fn parse_extern(&mut self) -> Result<Extern> {
        self.next_with_type(Types::KEYWORD(Keyword::FUNCTION))?;
        let name = self.next_with_type(Types::IDENTIFIER_FUNC)?;
        let args = self.parse_function_args()?;
        let return_type = self.parse_datatype().ok();

        Ok(Extern {
            name: name.value.unwrap(),
            args,
            return_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::{
        ASTNodes, Block, Expression, Extern, Function, FunctionCall, LetStmt, Literal,
    };

    use super::*;
    use lexer::{lexer::Lexer, types::Datatype};

    #[test]
    fn test_parse_import_def() {
        let mut lexer = Lexer::new("import std::io ");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::ImportDef(ImportDef {
                path: vec!["std".to_string(), "io".to_string()]
            })]
        );
    }

    #[test]
    fn test_parse_import_call() {
        let mut lexer = Lexer::new("func main() u32 { let u32 a = std::io::println(\"Test\", 4)}");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Function(Function {
                name: "main".to_string(),
                args: vec![],
                return_type: Some(Datatype::U32),
                body: Block {
                    body: vec![ASTNodes::LetStmt(LetStmt {
                        name: "a".to_string(),
                        value: Expression::Simple {
                            left: Box::new(ASTNodes::ImportCall(ImportCall {
                                path: vec![
                                    "std".to_string(),
                                    "io".to_string(),
                                    "println".to_string()
                                ],
                                ident: Box::new(ASTNodes::FunctionCall(FunctionCall {
                                    name: "println".to_string(),
                                    args: vec![
                                        Expression::String("Test".to_string()),
                                        Expression::Simple {
                                            left: Box::new(ASTNodes::Literal(Literal {
                                                value: "4".to_string(),
                                                r#type: Types::NUMBER
                                            })),
                                            right: None,
                                            operator: None
                                        }
                                    ]
                                }))
                            })),
                            right: None,
                            operator: None
                        },
                        datatype: Datatype::U32,
                        mutable: false
                    })]
                }
            })]
        );
    }

    #[test]
    fn test_extern_func() {
        let mut lexer = Lexer::new("extern func add(a u32, b u32) u32 ");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Extern(Extern {
                name: "add".to_string(),
                args: vec![
                    ("a".to_string(), Datatype::U32),
                    ("b".to_string(), Datatype::U32)
                ],
                return_type: Some(Datatype::U32),
            })]
        );
    }
}

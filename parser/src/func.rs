use lexer::types::{Datatype, Delimiter, Types};

use crate::nodes::{Expression, FunctionCall};

use super::{
    Parser, Result,
    errors::ParserError,
    nodes::{Function, Return},
};

impl Parser {
    pub(crate) fn parse_function_def(&mut self) -> Result<Function> {
        let name = self.next_with_type(Types::IDENTIFIER_FUNC)?;
        let args = self.parse_function_args()?;
        let return_type = if self
            .peek_if_type(Types::DELIMITER(Delimiter::LBRACE))
            .is_some()
        {
            None
        } else {
            Some(self.parse_datatype()?)
        };
        let body = self.parse_scoped_block()?;

        Ok(Function {
            name: name.value.unwrap(),
            args,
            return_type,
            body,
        })
    }

    pub(crate) fn parse_function_args(&mut self) -> Result<Vec<(String, Datatype)>> {
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

    pub(crate) fn parse_return(&mut self) -> Result<Return> {
        let val = self.parse_expression(vec![Types::DELIMITER(Delimiter::RBRACE), Types::NL])?;
        if val == Expression::None {
            return Ok(Return { value: None });
        }
        return Ok(Return { value: Some(val) });
    }

    pub(crate) fn parse_function_call(&mut self) -> Result<FunctionCall> {
        let name = self.current_with_type(Types::IDENTIFIER_FUNC)?;

        self.next_with_type(Types::DELIMITER(Delimiter::LPAREN))?;
        if self
            .next_if_type(Types::DELIMITER(Delimiter::RPAREN))
            .is_some()
        {
            return Ok(FunctionCall {
                name: name.value.unwrap(),
                args: vec![],
            });
        }

        let mut args = vec![];
        loop {
            let expr = self.parse_expression(vec![
                Types::DELIMITER(Delimiter::COMMA),
                Types::DELIMITER(Delimiter::RPAREN),
            ])?;
            args.push(expr);
            if self
                .next_if_type(Types::DELIMITER(Delimiter::RPAREN))
                .is_some()
            {
                break;
            }
            self.next_with_type(Types::DELIMITER(Delimiter::COMMA))?;
        }

        return Ok(FunctionCall {
            name: name.value.unwrap(),
            args,
        });
    }
}

#[cfg(test)]
mod tests {

    use crate::nodes::{Expression, LetStmt, Literal};

    use super::{
        super::nodes::{ASTNodes, Block},
        *,
    };

    use lexer::{lexer::Lexer, types::Datatype};

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
                return_type: Some(Datatype::U32),
                body: Block { body: vec![] },
            })]
        );
    }

    #[test]
    fn test_parse_function_def_no_ret() {
        let mut lexer = Lexer::new("func main() {}");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Function(Function {
                name: "main".to_string(),
                args: vec![],
                return_type: None,
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
                return_type: Some(Datatype::U32),
                body: Block {
                    body: vec![ASTNodes::Return(Return { value: None })]
                },
            }),]
        );
    }

    #[test]
    fn test_parse_function_def_with_return_val() {
        let mut lexer = Lexer::new("func main() u32 { return 5 }");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Function(Function {
                name: "main".to_string(),
                args: vec![],
                return_type: Some(Datatype::U32),
                body: Block {
                    body: vec![ASTNodes::Return(Return {
                        value: Some(Expression::Simple {
                            left: Box::new(ASTNodes::Literal(Literal {
                                value: "5".to_string(),
                                r#type: Types::NUMBER
                            })),
                            right: None,
                            operator: None
                        })
                    })]
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
                return_type: Some(Datatype::U32),
                body: Block { body: vec![] },
            })]
        );
    }

    #[test]
    fn test_parse_function_call() {
        let mut lexer = Lexer::new("func main() u32 { call() }");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Function(Function {
                name: "main".to_string(),
                args: vec![],
                return_type: Some(Datatype::U32),
                body: Block {
                    body: vec![ASTNodes::FunctionCall(FunctionCall {
                        name: "call".to_string(),
                        args: vec![]
                    })]
                },
            })]
        );
    }

    #[test]
    fn test_parse_function_call_2() {
        let mut lexer = Lexer::new("func main() u32 { let u32 a = call(4) }");
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
                            left: Box::new(ASTNodes::FunctionCall(FunctionCall {
                                name: "call".to_string(),
                                args: vec![Expression::Simple {
                                    left: Box::new(ASTNodes::Literal(Literal {
                                        value: "4".to_string(),
                                        r#type: Types::NUMBER
                                    })),
                                    right: None,
                                    operator: None
                                }]
                            })),
                            right: None,
                            operator: None
                        },
                        datatype: Datatype::U32,
                        mutable: false
                    }),]
                },
            })]
        );
    }
}

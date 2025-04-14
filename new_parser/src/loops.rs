use lexer::types::{Delimiter, Keyword, Operator, Types};

use crate::{
    Parser, Result,
    nodes::{ASTNodes, ForLoop, Loop},
};

impl Parser {
    pub fn parse_loop(&mut self) -> Result<ASTNodes> {
        if self.next_if_type(Types::KEYWORD(Keyword::RANGE)).is_some() {
            return self.parse_for_loop().map(|f| ASTNodes::ForLoop(f));
        }
        if self
            .peek_if_type(Types::DELIMITER(Delimiter::LBRACE))
            .is_some()
        {
            return Ok(ASTNodes::Loop(Loop {
                condition: None,
                body: self.parse_scoped_block()?,
            }));
        }

        let condition = self.parse_expression(vec![Types::DELIMITER(Delimiter::LBRACE)])?;
        let body = self.parse_scoped_block()?;

        Ok(ASTNodes::Loop(Loop {
            condition: Some(condition),
            body,
        }))
    }

    pub fn parse_for_loop(&mut self) -> Result<ForLoop> {
        let value = self.parse_variable()?;
        self.next_with_type(Types::DELIMITER(Delimiter::COMMA))?;
        let increment = self.parse_variable()?;
        self.next_with_type(Types::OPERATOR(Operator::ASSIGN))?;
        let iterator = self.parse_expression(vec![Types::DELIMITER(Delimiter::LBRACE)])?;
        Ok(ForLoop {
            value,
            increment,
            iterator,
            body: self.parse_scoped_block()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use lexer::{
        lexer::Lexer,
        types::{Datatype, Operator},
    };

    use crate::nodes::{
        ASTNodes, AssignStmt, Block, Expression, Function, Literal, Return, Variable,
    };

    use super::*;

    #[test]
    fn test_parse_loop() {
        let mut lexer = Lexer::new("func main() u32 { loop { return 1 } }");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Function(Function {
                name: "main".to_string(),
                args: vec![],
                return_type: Datatype::U32,
                body: Block {
                    body: vec![ASTNodes::Loop(Loop {
                        condition: None,
                        body: Block {
                            body: vec![ASTNodes::Return(Return {
                                value: Some(Expression::Simple {
                                    left: Box::new(ASTNodes::Literal(Literal {
                                        value: "1".to_string(),
                                        r#type: lexer::types::Types::NUMBER
                                    })),
                                    right: None,
                                    operator: None
                                })
                            })]
                        },
                    })]
                },
            })]
        )
    }

    #[test]
    fn test_parse_conditional_loop() {
        let mut lexer = Lexer::new("func main() u32 { loop 5 > 4 { return 1 } }");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Function(Function {
                name: "main".to_string(),
                args: vec![],
                return_type: Datatype::U32,
                body: Block {
                    body: vec![ASTNodes::Loop(Loop {
                        condition: Some(Expression::Simple {
                            left: Box::new(ASTNodes::Literal(Literal {
                                value: "5".to_string(),
                                r#type: lexer::types::Types::NUMBER
                            })),
                            right: Some(Box::new(ASTNodes::Literal(Literal {
                                value: "4".to_string(),
                                r#type: lexer::types::Types::NUMBER
                            }))),
                            operator: Some(lexer::types::Operator::GREATER)
                        }),
                        body: Block {
                            body: vec![ASTNodes::Return(Return {
                                value: Some(Expression::Simple {
                                    left: Box::new(ASTNodes::Literal(Literal {
                                        value: "1".to_string(),
                                        r#type: lexer::types::Types::NUMBER
                                    })),
                                    right: None,
                                    operator: None
                                })
                            })]
                        },
                    })]
                },
            })]
        )
    }

    #[test]
    fn test_parse_for_loop() {
        let mut lexer =
            Lexer::new("func main() u32 { loop range val, index = array { a = i * 2 } }");
        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Function(Function {
                name: "main".to_string(),
                args: vec![],
                return_type: Datatype::U32,
                body: Block {
                    body: vec![ASTNodes::ForLoop(ForLoop {
                        value: Variable {
                            name: "val".to_string(),
                        },
                        increment: Variable {
                            name: "index".to_string(),
                        },
                        iterator: Expression::Simple {
                            left: Box::new(ASTNodes::Variable(Variable {
                                name: "array".to_string(),
                            })),
                            right: None,
                            operator: None
                        },
                        body: Block {
                            body: vec![ASTNodes::AssignStmt(AssignStmt {
                                name: "a".to_string(),
                                value: Expression::Simple {
                                    left: Box::new(ASTNodes::Variable(Variable {
                                        name: "i".to_string(),
                                    })),
                                    right: Some(Box::new(ASTNodes::Literal(Literal {
                                        value: "2".to_string(),
                                        r#type: lexer::types::Types::NUMBER
                                    }))),
                                    operator: Some(Operator::MULTIPLY)
                                },
                            })]
                        }
                    })]
                },
            })]
        )
    }
}

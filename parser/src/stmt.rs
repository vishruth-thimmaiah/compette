use lexer::types::{Datatype, Delimiter, Operator, Types};

use crate::{
    Parser, Result,
    nodes::{AssignStmt, Expression, LetStmt, StructDef},
};

impl Parser {
    pub(crate) fn parse_statement(&mut self) -> Result<LetStmt> {
        let mut datatype = self.parse_datatype()?;
        let mutable = self.next_if_type(Types::OPERATOR(Operator::NOT)).is_some();
        let name = self.next_with_type(Types::IDENTIFIER)?;
        self.next_with_type(Types::OPERATOR(Operator::ASSIGN))?;
        let value = self.parse_expression(vec![Types::NL, Types::DELIMITER(Delimiter::RBRACE)])?;

        datatype = self.update_arr_datatype(datatype, &value);

        Ok(LetStmt {
            name: name.value.unwrap(),
            value,
            datatype,
            mutable,
        })
    }

    fn update_arr_datatype(&mut self, mut dt: Datatype, arr: &Expression) -> Datatype {
        if let Expression::Array(arr) = arr {
            if let Datatype::NARRAY(mut inner, _) = dt {
                inner = Box::new(self.update_arr_datatype(*inner, arr.get(0).unwrap()));
                dt = Datatype::NARRAY(inner, arr.len());
            }
        } else if let Expression::String(string) = arr {
            if let Datatype::STRING(size) = dt {
                dt = Datatype::STRING(string.len() + size);
            } else {
                dt = Datatype::CSTRING(string.len());
            }
        }
        dt
    }

    pub(crate) fn parse_struct_def(&mut self) -> Result<StructDef> {
        let name = self.next_with_type(Types::IDENTIFIER)?;

        self.next_with_type(Types::DELIMITER(Delimiter::LBRACE))?;
        let mut args = vec![];
        loop {
            let name = self.next_with_type(Types::IDENTIFIER)?;
            let dt = self.parse_datatype()?;
            args.push((name.value.unwrap(), dt));

            if self
                .next_if_type(Types::DELIMITER(Delimiter::RBRACE))
                .is_some()
            {
                break;
            }
            self.next_with_type(Types::DELIMITER(Delimiter::COMMA))?;
        }

        return Ok(StructDef {
            name: name.value.unwrap(),
            fields: args,
        });
    }

    pub(crate) fn parse_assign_stmt(&mut self) -> Result<AssignStmt> {
        let name = self.parse_complex_variable()?;
        self.next_with_type(Types::OPERATOR(Operator::ASSIGN))?;
        let value = self.parse_expression(vec![Types::NL, Types::DELIMITER(Delimiter::RBRACE)])?;

        Ok(AssignStmt {
            name: Box::new(name),
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use lexer::{lexer::Lexer, types::Datatype};

    use crate::nodes::{ASTNodes, Attr, Block, Expression, Function, LetStmt, Literal, Variable};

    use super::*;

    #[test]
    fn test_parse_statement() {
        let mut lexer = Lexer::new("func main() u32 { let u32 a = 1 }");

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
                            left: Box::new(ASTNodes::Literal(Literal {
                                value: "1".to_string(),
                                r#type: lexer::types::Types::NUMBER
                            })),
                            right: None,
                            operator: None
                        },
                        datatype: Datatype::U32,
                        mutable: false
                    })]
                },
            })]
        );
    }

    #[test]
    fn test_parse_mut_statement() {
        let mut lexer = Lexer::new("func main() u32 { let u32! b = a }");

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
                        name: "b".to_string(),
                        value: Expression::Simple {
                            left: Box::new(ASTNodes::Variable(Variable {
                                name: "a".to_string(),
                            })),
                            right: None,
                            operator: None
                        },
                        datatype: Datatype::U32,
                        mutable: true
                    })]
                },
            })]
        );
    }

    #[test]
    fn test_parse_struct_def() {
        let mut lexer = Lexer::new("struct Test { a u32, b u32 }");

        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::StructDef(StructDef {
                name: "Test".to_string(),
                fields: vec![
                    ("a".to_string(), Datatype::U32),
                    ("b".to_string(), Datatype::U32)
                ]
            })]
        );
    }

    #[test]
    fn test_parse_assign_stmt() {
        let mut lexer = Lexer::new("func main() u32 { a = 1 + 7 }");

        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Function(Function {
                name: "main".to_string(),
                args: vec![],
                return_type: Some(Datatype::U32),
                body: Block {
                    body: vec![ASTNodes::AssignStmt(AssignStmt {
                        name: Box::new(ASTNodes::Variable(Variable {
                            name: "a".to_string(),
                        })),
                        value: Expression::Simple {
                            left: Box::new(ASTNodes::Literal(Literal {
                                value: "1".to_string(),
                                r#type: lexer::types::Types::NUMBER
                            })),
                            right: Some(Box::new(ASTNodes::Literal(Literal {
                                value: "7".to_string(),
                                r#type: lexer::types::Types::NUMBER
                            }))),
                            operator: Some(Operator::PLUS)
                        },
                    })]
                },
            })]
        );
    }

    #[test]
    fn test_parse_assign_to_attr() {
        let mut lexer = Lexer::new("func main() u32 { a.b = 1 + 7 }");

        let mut parser = Parser::new(lexer.tokenize());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            vec![ASTNodes::Function(Function {
                name: "main".to_string(),
                args: vec![],
                return_type: Some(Datatype::U32),
                body: Block {
                    body: vec![ASTNodes::AssignStmt(AssignStmt {
                        name: Box::new(ASTNodes::Attr(Attr {
                            name: Variable {
                                name: "b".to_string()
                            },
                            parent: Box::new(ASTNodes::Variable(Variable {
                                name: "a".to_string()
                            }))
                        })),
                        value: Expression::Simple {
                            left: Box::new(ASTNodes::Literal(Literal {
                                value: "1".to_string(),
                                r#type: lexer::types::Types::NUMBER
                            })),
                            right: Some(Box::new(ASTNodes::Literal(Literal {
                                value: "7".to_string(),
                                r#type: lexer::types::Types::NUMBER
                            }))),
                            operator: Some(Operator::PLUS)
                        },
                    })]
                },
            })]
        );
    }
}

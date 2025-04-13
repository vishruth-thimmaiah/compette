use lexer::types::{Delimiter, Operator, Types};

use crate::{Parser, Result, nodes::LetStmt};

impl Parser {
    pub(crate) fn parse_statement(&mut self) -> Result<LetStmt> {
        let datatype = self.parse_datatype()?;
        let mutable = self.next_if_type(Types::OPERATOR(Operator::NOT)).is_some();
        let name = self.next_with_type(Types::IDENTIFIER)?;
        self.next_with_type(Types::OPERATOR(Operator::ASSIGN))?;
        let value = self.parse_expression(vec![Types::NL, Types::DELIMITER(Delimiter::RBRACE)])?;

        Ok(LetStmt {
            name: name.value.unwrap(),
            value,
            datatype,
            mutable,
        })
    }
}

#[cfg(test)]
mod tests {
    use lexer::{lexer::Lexer, types::Datatype};

    use crate::nodes::{ASTNodes, Block, Expression, Function, LetStmt, Literal, Variable};

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
                return_type: Datatype::U32,
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
                return_type: Datatype::U32,
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
}

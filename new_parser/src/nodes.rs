use lexer::types::{Datatype, Operator, Types};

#[derive(Debug, PartialEq)]
pub enum ASTNodes {
    Block(Block),
    Expression(Expression),
    Function(Function),
    Literal(Literal),
    Token(Types),
    Return(Return),
    Variable(Variable),
    LetStmt(LetStmt),
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub args: Vec<(String, Datatype)>,
    pub return_type: Datatype,
    pub body: Block,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub body: Vec<ASTNodes>,
}

#[derive(Debug, PartialEq)]
pub struct Return {
    pub value: Option<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Simple {
        left: Box<ASTNodes>,
        right: Option<Box<ASTNodes>>,
        operator: Option<Operator>,
    },
    Array(Vec<Expression>),
    String(String),
    Struct(Vec<(String, Expression)>),
    None,
}

#[derive(Debug, PartialEq)]
pub struct Literal {
    pub value: String,
    pub r#type: Types,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct LetStmt {
    pub name: String,
    pub value: Expression,
    pub datatype: Datatype,
    pub mutable: bool,
}

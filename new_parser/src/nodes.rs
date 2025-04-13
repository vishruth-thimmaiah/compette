use lexer::types::{Datatype, Operator, Types};

#[derive(Debug, PartialEq)]
pub enum ASTNodes {
    Block(Block),
    Expression(Expression),
    Function(Function),
    FunctionCall(FunctionCall),
    ImportCall(ImportCall),
    Literal(Literal),
    Token(Types),
    Return(Return),
    Variable(Variable),
    LetStmt(LetStmt),
    AssignStmt(AssignStmt),
    StructDef(StructDef),
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

impl Expression {
    pub fn is_none(&self) -> bool {
        if let Expression::None = self {
            return true;
        }
        false
    }
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

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expression>,
    // pub imported: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<(String, Datatype)>,
}

#[derive(Debug, PartialEq)]
pub struct ImportCall {
    pub path: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct AssignStmt {
    pub name: String,
    pub value: Expression,
}

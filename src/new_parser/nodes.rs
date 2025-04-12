use crate::lexer::types::DATATYPE;

#[derive(Debug, PartialEq)]
pub enum ASTNodes {
    Function(Function),
    Block(Block),
}


#[derive(Debug, PartialEq)]
pub struct Function {
    pub name:  String,
    pub args: Vec<(String, DATATYPE)>,
    pub return_type: DATATYPE,
    pub body: Block,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub body: Vec<ASTNodes>,
}

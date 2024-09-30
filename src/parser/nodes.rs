use std::fmt::Debug;

use crate::lexer::{lexer::Token, types::Types};

use super::types::ParserTypes;

pub trait ParserType: Debug {
    fn get_type(&self) -> ParserTypes;
}

#[derive(Debug)]
pub struct AssignmentParserNode {
    pub var_name: String,
    pub value: Box<dyn ParserType>,
}
impl ParserType for AssignmentParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::VARIABLE
    }
}

#[derive(Debug)]
pub struct ExpressionParserNode {
    pub left: Token,
    pub right: Option<Box<dyn ParserType>>,
    pub operator: Option<Types>,
}
impl ParserType for ExpressionParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::EXPRESSION
    }
}

#[derive(Debug)]
pub struct FunctionParserNode {
    pub func_name: String,
    pub args: Vec<String>,
    pub body: Vec<Box<dyn ParserType>>,
}
impl ParserType for FunctionParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::FUNCTION
    }
}

#[derive(Debug)]
pub struct FunctionCallParserNode {
    pub func_name: String,
    pub args: Vec<String>,
}
impl ParserType for FunctionCallParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::FUNCTION_CALL
    }
}

#[derive(Debug)]
pub struct ConditionalIfParserNode {
    pub condition: ExpressionParserNode,
    pub body: Vec<Box<dyn ParserType>>,
    pub else_if_body: Vec<ConditionalElseIfParserNode>,
    pub else_body: Option<ConditionalElseParserNode>,
}
impl ParserType for ConditionalIfParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::CONDITIONAL
    }
}

#[derive(Debug)]
pub struct ConditionalElseIfParserNode {
    pub condition: ExpressionParserNode,
    pub body: Vec<Box<dyn ParserType>>,
}
impl ParserType for ConditionalElseIfParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::CONDITIONAL
    }
}


#[derive(Debug)]
pub struct ConditionalElseParserNode {
    pub body: Vec<Box<dyn ParserType>>,
}
impl ParserType for ConditionalElseParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::CONDITIONAL
    }
}

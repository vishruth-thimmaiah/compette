use std::{any::Any, fmt::Debug};

use crate::lexer::{lexer::Token, types::Types};

use super::types::ParserTypes;

#[derive(Debug)]
pub struct ParserToken {
    pub value: String,
    pub r#type: Types,
}

impl ParserToken {
    pub fn from(token: Token) -> Self {
        if None == token.value {
            panic!("invalid Token")
        }
        Self {
            value: token.value.unwrap(),
            r#type: token.r#type,
        }
    }
}

pub trait ParserType: Debug {
    fn get_type(&self) -> ParserTypes;
    fn any(&self) -> &dyn Any;
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
    fn any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct ExpressionParserNode {
    pub left: ParserToken,
    pub right: Option<Box<dyn ParserType>>,
    pub operator: Option<Types>,
}
impl ParserType for ExpressionParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::EXPRESSION
    }
    fn any(&self) -> &dyn Any {
        self
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
    fn any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct ReturnNode {
    pub return_value: Box<dyn ParserType>,
}
impl ParserType for ReturnNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::RETURN
    }
    fn any(&self) -> &dyn Any {
        self
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
    fn any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct VariableCallParserNode {
    pub var_name: String,
    pub rhs: Box<ExpressionParserNode>,
}
impl ParserType for VariableCallParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::VARIABLE_CALL
    }
    fn any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct ConditionalIfParserNode {
    pub condition: Box<ExpressionParserNode>,
    pub body: Vec<Box<dyn ParserType>>,
    pub else_if_body: Vec<ConditionalElseIfParserNode>,
    pub else_body: Option<ConditionalElseParserNode>,
}
impl ParserType for ConditionalIfParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::CONDITIONAL
    }
    fn any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct ConditionalElseIfParserNode {
    pub condition: Box<ExpressionParserNode>,
    pub body: Vec<Box<dyn ParserType>>,
}
impl ParserType for ConditionalElseIfParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::CONDITIONAL
    }
    fn any(&self) -> &dyn Any {
        self
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
    fn any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct LoopParserNode {
    pub condition: Box<ExpressionParserNode>,
    pub body: Vec<Box<dyn ParserType>>,
}
impl ParserType for LoopParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::LOOP
    }
    fn any(&self) -> &dyn Any {
        self
    }
}

use std::{any::Any, fmt::Debug};

use crate::lexer::types::{Types, DATATYPE, OPERATOR};

use super::types::ParserTypes;

pub trait ParserType: Debug {
    fn get_type(&self) -> ParserTypes;
    fn any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct AssignmentParserNode {
    pub var_name: String,
    pub var_type: DATATYPE,
    pub is_mutable: bool,
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
    pub left: Box<dyn ParserType>,
    pub right: Option<Box<dyn ParserType>>,
    pub operator: Option<OPERATOR>,
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
    pub args: Vec<(String, DATATYPE)>,
    pub return_type: Option<DATATYPE>,
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
pub struct BreakNode {}
impl ParserType for BreakNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::BREAK
    }
    fn any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct FunctionCallParserNode {
    pub func_name: String,
    pub args: Vec<ExpressionParserNode>,
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
    pub var_name: Box<dyn ParserType>,
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
pub struct ValueParserNode {
    pub value: String,
    pub r#type: Types,
}
impl ParserType for ValueParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::VALUE
    }
    fn any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct ValueIterCallParserNode {
    pub value: String,
    pub index: Box<ExpressionParserNode>,
}
impl ParserType for ValueIterCallParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::VALUE_ITER_CALL
    }
    fn any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct ValueIterParserNode {
    pub value: Vec<ExpressionParserNode>,
}
impl ParserType for ValueIterParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::VALUE_ITER
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

#[derive(Debug)]
pub struct ForLoopParserNode {
    pub iterator: Box<dyn ParserType>,
    pub index: String,
    pub incr_value: String,
    pub body: Vec<Box<dyn ParserType>>,
}
impl ParserType for ForLoopParserNode {
    fn get_type(&self) -> ParserTypes {
        ParserTypes::FOR_LOOP
    }
    fn any(&self) -> &dyn Any {
        self
    }
}

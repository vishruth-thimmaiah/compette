#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum ParserTypes {
    CONDITIONAL,
    LOOP,
    FUNCTION,
    FUNCTION_CALL,
    VARIABLE,
    EXPRESSION,
}

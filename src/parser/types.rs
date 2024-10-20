#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ParserTypes {
    CONDITIONAL,
    LOOP,
    FUNCTION,
    FUNCTION_CALL,
    RETURN,
    VARIABLE,
    EXPRESSION,
    VARIABLE_CALL,
    VALUE,
}

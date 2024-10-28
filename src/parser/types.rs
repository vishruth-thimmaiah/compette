#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ParserTypes {
    CONDITIONAL,
    LOOP,
    FOR_LOOP,
    FUNCTION,
    FUNCTION_CALL,
    RETURN,
    VARIABLE,
    EXPRESSION,
    VARIABLE_CALL,
    VALUE,
    VALUE_ITER,
    VALUE_ITER_CALL,
}

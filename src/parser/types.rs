#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ParserTypes {
    IMPORT,
    STRUCT,
    CONDITIONAL,
    LOOP,
    FOR_LOOP,
    FUNCTION,
    FUNCTION_CALL,
    RETURN,
    BREAK,
    VARIABLE,
    EXPRESSION,
    VARIABLE_CALL,
    VALUE,
    VALUE_ITER,
    VALUE_ITER_CALL,
}

#[derive(Debug, PartialEq)]
pub enum ParserTypes {
    CONDITIONAL,
    LOOP,
    FUNCTION,
    FUNCTION_CALL,
    VARIABLE,
    EXPRESSION,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum Types {
    NL,
    EOF,
    OPERATOR(OPERATOR),
    DELIMITER(DELIMITER),
    KEYWORD(KEYWORD),
    IDENTIFIER,
    NUMBER,
    BOOL,
    IDENTIFIER_FUNC,
    DATATYPE(DATATYPE),
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum OPERATOR {
    ASSIGN,
    NOT,
    DOT,
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    EQUAL,
    LESSER,
    GREATER,
    LESSER_EQUAL,
    GREATER_EQUAL,
    NOT_EQUAL,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DELIMITER {
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
}

#[derive(Debug, PartialEq, Clone)]
pub enum KEYWORD {
    FUNCTION,
    LET,
    RETURN,
    IF,
    ELSE,
    LOOP,
    RANGE,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DATATYPE {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    BOOL,
    STRING(usize),
    ARRAY(Box<ArrayDetails>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayDetails {
    pub datatype: DATATYPE,
    pub length: u32,
}

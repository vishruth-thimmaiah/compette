#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum Types {
    NL,
    EOF,
    OPERATOR(Operator),
    DELIMITER(Delimiter),
    KEYWORD(Keyword),
    IDENTIFIER,
    IDENTIFIER_FUNC,
    NUMBER,
    BOOL,
    DATATYPE(Datatype),
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
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
    COLON,
    CAST, // ->
    PATH, // ::
}

#[derive(Debug, PartialEq, Clone)]
pub enum Delimiter {
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
pub enum Keyword {
    IMPORT,
    STRUCT,
    FUNCTION,
    LET,
    RETURN,
    IF,
    ELSE,
    LOOP,
    RANGE,
    BREAK,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Datatype {
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
    NARRAY(Box<Datatype>, usize),
    CUSTOM(String),
    NONE,
}

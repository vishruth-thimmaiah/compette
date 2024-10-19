#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum Types {
    NL,
    EOF,

    // Operators
    ASSIGN,
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

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    RETURN,
    IF,
    ELSE,
    LOOP,

    // Identifiers
    IDENTIFIER,
    NUMBER,

    // Types
    U16,
    U32,
    U64,
    I16,
    I32,
    I64,
    F32,
    F64,
    BOOL,
    STRING,
}

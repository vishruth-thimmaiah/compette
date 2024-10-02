#[derive(Debug, PartialEq, Clone)]
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

    // Identifiers
    IDENTIFIER,
    NUMBER,
    STRING,
}

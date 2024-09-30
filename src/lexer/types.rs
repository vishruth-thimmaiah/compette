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

    // Identifiers
    IDENTIFIER,
    NUMBER,
    STRING,
}

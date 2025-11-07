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
    MODULO,
    EQUAL,
    LESSER,
    GREATER,
    LESSER_EQUAL,
    GREATER_EQUAL,
    NOT_EQUAL,
    COLON,
    CAST, // ->
    PATH, // ::
    BITWISE_AND,
    BITWISE_OR,
    BITWISE_XOR,
    LSHIFT,
    RSHIFT,
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
    EXTERN,
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
    CSTRING(usize),
    NARRAY(Box<Datatype>, usize),
    SIMD(Box<Datatype>, usize),
    CUSTOM(String),
    NONE,
}

impl Datatype {
    pub fn is_trivial(&self) -> bool {
        match self {
            Datatype::U8 | Datatype::U16 | Datatype::U32 | Datatype::U64 => true,
            Datatype::I8 | Datatype::I16 | Datatype::I32 | Datatype::I64 => true,
            Datatype::F32 | Datatype::F64 => true,
            Datatype::BOOL => true,
            _ => false,
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            Datatype::U8 | Datatype::I8 => 1,
            Datatype::U16 | Datatype::I16 => 2,
            Datatype::U32 | Datatype::I32 => 4,
            Datatype::U64 | Datatype::I64 => 8,
            Datatype::F32 => 4,
            Datatype::F64 => 8,
            Datatype::BOOL => 1,
            Datatype::STRING(s) => *s,
            Datatype::CSTRING(s) => *s,
            Datatype::NARRAY(ty, s) => ty.get_size() * s,
            Datatype::SIMD(ty, s) => ty.get_size() * s,
            Datatype::CUSTOM(_) => unimplemented!(),
            Datatype::NONE => 0,
        }
    }
}

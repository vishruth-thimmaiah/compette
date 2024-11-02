use crate::lexer::types::DATATYPE;

pub struct Func {
    pub name: &'static str,
    pub args: &'static [(&'static str, &'static DATATYPE)],
    pub return_type: &'static DATATYPE,
}

pub const SUPPORTED_FUNCS: &[Func] = &[
    Func {
        name: "print",
        args: &[("s", &DATATYPE::STRING(0))],
        return_type: &DATATYPE::NONE,
    },
    Func {
        name: "println",
        args: &[("s", &DATATYPE::STRING(0))],
        return_type: &DATATYPE::NONE,
    },
];

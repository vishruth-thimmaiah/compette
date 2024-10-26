use crate::lexer::types::DATATYPE;

pub struct Func {
    pub name: &'static str,
    pub args: &'static [(&'static str, &'static DATATYPE)],
    pub return_type: &'static DATATYPE,
}

pub const SUPPORTED_FUNCS: &[Func] = &[
    Func {
        name: "add",
        args: &[("a", &DATATYPE::U32), ("b", &DATATYPE::U32)],
        return_type: &DATATYPE::U32,
    },
    Func {
        name: "print",
        args: &[("s", &DATATYPE::STRING(0))],
        return_type: &DATATYPE::U32,
    },
    Func {
        name: "println",
        args: &[("s", &DATATYPE::STRING(0))],
        return_type: &DATATYPE::U32,
    },
    Func {
        name: "test",
        args: &[("s", &DATATYPE::STRING(0))],
        return_type: &DATATYPE::U32,
    },
];

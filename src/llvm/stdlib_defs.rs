use crate::lexer::types::DATATYPE;

#[allow(dead_code)]
pub struct StdLibModule {
    pub name: &'static str,
    pub funcs: &'static [StdLibFunc],
    pub sub_modules: &'static [StdLibModule],
}

pub struct StdLibFunc {
    pub name: &'static str,
    pub args: &'static [(&'static str, &'static DATATYPE)],
    pub return_type: &'static DATATYPE,
}

pub const STDLIB_MODULES: &[StdLibModule] = &[StdLibModule {
    name: "io",
    funcs: &[
        StdLibFunc {
            name: "print",
            args: &[("s", &DATATYPE::STRING(0))],
            return_type: &DATATYPE::NONE,
        },
        StdLibFunc {
            name: "println",
            args: &[("s", &DATATYPE::STRING(0))],
            return_type: &DATATYPE::NONE,
        },
        // Temporary funtion until format print is implemented
        StdLibFunc {
            name: "printint",
            args: &[("s", &DATATYPE::I32)],
            return_type: &DATATYPE::NONE,
        },
        StdLibFunc {
            name: "printfl",
            args: &[("s", &DATATYPE::F32)],
            return_type: &DATATYPE::NONE,
        },
    ],
    sub_modules: &[],
}];

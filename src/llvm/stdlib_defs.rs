use crate::lexer::types::DATATYPE;

#[allow(dead_code)]
pub struct StdLibFunc {
    pub args: &'static [(&'static str, &'static DATATYPE)],
    pub return_type: &'static DATATYPE,
    pub ptr: usize,
}

pub fn get_stdlib_function(name: &str) -> Option<StdLibFunc> {
    let func = match name {
        "__std__io__print" => StdLibFunc {
            args: &[("s", &DATATYPE::STRING(0))],
            return_type: &DATATYPE::NONE,
            ptr: stdlib::io::__std__io__print as usize,
        },
        "__std__io__println" => StdLibFunc {
            args: &[("s", &DATATYPE::STRING(0))],
            return_type: &DATATYPE::NONE,
            ptr: stdlib::io::__std__io__println as usize,
        },
        // Temporary funtion until format print is implemented
        "__std__io__printint" => StdLibFunc {
            args: &[("s", &DATATYPE::I32)],
            return_type: &DATATYPE::NONE,
            ptr: stdlib::io::__std__io__printint as usize,
        },
        "__std__io__printflt" => StdLibFunc {
            args: &[("s", &DATATYPE::F32)],
            return_type: &DATATYPE::NONE,
            ptr: stdlib::io::__std__io__printflt as usize,
        },
        "__std__builtin__len" => StdLibFunc {
            args: &[("arr", &DATATYPE::U32)],
            ptr: stdlib::builtin::arrays::len as usize,
            return_type: &DATATYPE::U64,
        },
        _ => return None,
    };
    Some(func)
}

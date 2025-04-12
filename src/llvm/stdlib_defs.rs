use crate::lexer::types::Datatype;

#[allow(dead_code)]
pub struct StdLibFunc {
    pub args: &'static [(&'static str, &'static Datatype)],
    pub return_type: &'static Datatype,
    pub ptr: usize,
}

pub fn get_stdlib_function(name: &str) -> Option<StdLibFunc> {
    let func = match name {
        "__std__io__print" => StdLibFunc {
            args: &[("s", &Datatype::STRING(0))],
            return_type: &Datatype::NONE,
            ptr: stdlib::io::__std__io__print as usize,
        },
        "__std__io__println" => StdLibFunc {
            args: &[("s", &Datatype::STRING(0))],
            return_type: &Datatype::NONE,
            ptr: stdlib::io::__std__io__println as usize,
        },
        // Temporary funtion until format print is implemented
        "__std__io__printint" => StdLibFunc {
            args: &[("s", &Datatype::I32)],
            return_type: &Datatype::NONE,
            ptr: stdlib::io::__std__io__printint as usize,
        },
        "__std__io__printflt" => StdLibFunc {
            args: &[("s", &Datatype::F32)],
            return_type: &Datatype::NONE,
            ptr: stdlib::io::__std__io__printflt as usize,
        },
        _ => return None,
    };
    Some(func)
}

pub fn get_builtin_function(name: &str) -> Option<StdLibFunc> {
    let func = match name {
        "__builtin__len" => StdLibFunc {
            args: &[("arr", &Datatype::U32)],
            ptr: stdlib::builtin::arrays::__builtin__len as usize,
            return_type: &Datatype::U64,
        },
        _ => return None,
    };
    Some(func)
}

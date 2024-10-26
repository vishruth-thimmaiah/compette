use std::{
    ffi::{c_char, CStr},
    io::Write,
};

#[no_mangle]
extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
extern "C" fn print(s: *const c_char) -> i32 {
    let string = unsafe { CStr::from_ptr(s) };
    std::io::stdout().write_all(string.to_bytes()).unwrap();
    std::io::stdout().flush().unwrap();
    return 0;
}

#[no_mangle]
extern "C" fn println(s: *const c_char) -> i32 {
    let string = unsafe { CStr::from_ptr(s) };
    std::io::stdout().write_all(string.to_bytes()).unwrap();
    std::io::stdout().write(b"\n").unwrap();
    return 0;
}

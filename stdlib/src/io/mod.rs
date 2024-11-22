use std::{
    ffi::{c_char, CStr},
    io::Write,
};

#[no_mangle]
pub extern "C" fn print(s: *const c_char) {
    let string = unsafe { CStr::from_ptr(s) };
    std::io::stdout().write_all(string.to_bytes()).unwrap();
    std::io::stdout().flush().unwrap();
}

#[no_mangle]
pub extern "C" fn println(s: *const c_char) {
    let string = unsafe { CStr::from_ptr(s) };
    std::io::stdout().write_all(string.to_bytes()).unwrap();
    std::io::stdout().write(b"\n").unwrap();
}

// Temporary funtion until format print is implemented
#[no_mangle]
pub extern "C" fn printint(s: i32) {
    println!("{}", s);
}
#[no_mangle]
pub extern "C" fn printfl(s: f32) {
    println!("{}", s);
}

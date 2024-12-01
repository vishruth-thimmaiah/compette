use core::slice;
use std::io::Write;

#[derive(Debug)]
#[repr(C)]
pub struct Str {
    pub len: u64,
    pub str: *const u8,
}

#[no_mangle]
pub extern "C" fn __std__io__print(s: *const Str) {
    let str_struct = unsafe { &*s };
    let string = unsafe { slice::from_raw_parts(str_struct.str, str_struct.len as usize) };
    std::io::stdout().write_all(string).unwrap();
    std::io::stdout().flush().unwrap();
}

#[no_mangle]
pub extern "C" fn __std__io__println(s: *const Str) {
    let str_struct = unsafe { &*s };
    let string = unsafe { slice::from_raw_parts(str_struct.str, str_struct.len as usize) };
    std::io::stdout().write_all(string).unwrap();
    std::io::stdout().write(b"\n").unwrap();
}

// Temporary funtion until format print is implemented
#[no_mangle]
pub extern "C" fn __std__io__printint(s: i32) {
    println!("{}", s);
}
#[no_mangle]
pub extern "C" fn __std__io__printflt(s: f32) -> i32 {
    println!("{}", s);
    return 0;
}

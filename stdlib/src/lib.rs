pub const SUPPORTED_FUNCTIONS: &[&str] = &["add"];

#[no_mangle]
extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b + 10
}

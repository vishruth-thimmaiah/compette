#[repr(C)]
pub struct Array {
    pub len: u64,
    pub data: *const u32,
}

#[no_mangle]
pub extern "C" fn __builtin__len(arr: *const Array) -> u64 {
    let deref_arr = unsafe { &*(arr) };
    return deref_arr.len;
}

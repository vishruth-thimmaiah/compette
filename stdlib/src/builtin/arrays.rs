#[repr(C)]
pub struct Array {
    pub len: u64,
	pub data: *const u32,
}

#[no_mangle]
pub extern "C" fn __std__builtin__len(arr: Array) -> u64 {
    let len = unsafe { &*(arr.len as *const u64) };
    return *len;
}

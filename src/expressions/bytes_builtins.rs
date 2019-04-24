use libc::c_char;
use std::ffi::CStr;

#[repr(C)]
pub struct Bytes<'a> {
    pub values: &'a [u8],
    pub size: i64,
}

#[no_mangle]
pub extern "C" fn get_bytes(bytes: *mut Bytes, index: i64) -> u8 {
    unsafe { (*bytes).values[index as usize] }
}

#[no_mangle]
pub extern "C" fn print_bytes(bytes: *mut Bytes) {
    print!("{}", unsafe {
        String::from_utf8((*bytes).values.to_vec()).unwrap()
    });
}

#[no_mangle]
pub extern "C" fn print_byte(value: u8) {
    print!("{}", value as char);
}

#[no_mangle]
pub extern "C" fn len_bytes(bytes: *mut Bytes) -> i64 {
    unsafe { (*bytes).size }
}

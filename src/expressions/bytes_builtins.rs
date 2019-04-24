use libc::c_char;
use std::{ffi::CStr, slice, str};

#[repr(C)]
pub struct Bytes {
    pub values: *mut u8,
    pub size: i64,
}

#[no_mangle]
pub extern "C" fn get_bytes(bytes: *mut Bytes, index: i64) -> u8 {
    unsafe { slice::from_raw_parts((*bytes).values, (*bytes).size as usize)[index as usize] }
}

#[no_mangle]
pub extern "C" fn print_bytes(bytes: *mut Bytes) {
    print!("{}\n", unsafe {
        let bytes_in_rust = slice::from_raw_parts((*bytes).values, (*bytes).size as usize);
        str::from_utf8(bytes_in_rust).unwrap()
    });
}

#[no_mangle]
pub extern "C" fn print_byte(value: u8) {
    print!("{}\n", value as char);
}

#[no_mangle]
pub extern "C" fn len_bytes(bytes: *mut Bytes) -> i64 {
    unsafe { (*bytes).size }
}

use super::Array;
use std::{slice, str};

type Bytes = Array<u8>;

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

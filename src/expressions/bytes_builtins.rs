use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn get_bytes(value: *const c_char, index: i64) -> u8 {
    unsafe { CStr::from_ptr(value).to_bytes()[index as usize] }
}

#[no_mangle]
pub extern "C" fn print_bytes(value: *const c_char) {
    print!("{}", unsafe { CStr::from_ptr(value).to_str().unwrap() });
}

#[no_mangle]
pub extern "C" fn print_byte(value: u8) {
    print!("{}", value as char);
}

#[no_mangle]
pub extern "C" fn len_bytes(value: *const c_char) -> i64 {
    unsafe { CStr::from_ptr(value).to_str().unwrap().len() as i64 }
}

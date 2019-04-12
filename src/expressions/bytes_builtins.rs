use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn get_bytes(value: *const c_char, index: i64) -> u8 {
    unsafe { CStr::from_ptr(value).to_bytes()[index as usize] }
}

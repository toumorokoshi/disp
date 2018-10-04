use libc::c_char;
use std::ffi::{CStr, CString};

/// convert a string into an llvm compatible literal
pub fn to_ptr(s: &str) -> *const c_char {
    let c_string = CString::new(s.clone()).unwrap();
    c_string.into_raw()
}

pub fn to_string(s: *const c_char) -> String {
    unsafe { String::from(CStr::from_ptr(*&s).to_str().unwrap()) }
}

use super::to_ptr;
use std::ffi::CStr;

// This equality relationship is important,
// as it is used to determined the existence of keys
// in a set.
#[test]
fn test_pointer_equality() {
    let a = to_ptr("foo");
    let b = to_ptr("foo");
    let a_cstr = unsafe { Box::new(CStr::from_ptr(*&a)) };
    let b_cstr = unsafe { Box::new(CStr::from_ptr(*&b)) };
    assert_eq!(a_cstr, b_cstr);
}

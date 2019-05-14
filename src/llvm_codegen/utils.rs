use super::{CompilerData, FunctionType, Scope, Type};
use libc::c_char;
use llvm_sys::{core::*, prelude::*, *};
use std::{
    ffi::{CStr, CString},
    mem,
};

/// convert a string into an llvm compatible literal
pub fn to_ptr(s: &str) -> *const c_char {
    let c_string = CString::new(s.clone()).unwrap();
    c_string.into_raw()
}

pub fn to_string(s: *const c_char) -> String {
    unsafe { String::from(CStr::from_ptr(*&s).to_str().unwrap()) }
}

pub fn extract_type_from_pointer(typ: LLVMTypeRef) -> LLVMTypeRef {
    unsafe {
        let mut llvm_types: [LLVMTypeRef; 10] = [LLVMVoidType(); 10];
        LLVMGetSubtypes(typ, llvm_types.as_mut_ptr());
        return llvm_types[0];
    }
}
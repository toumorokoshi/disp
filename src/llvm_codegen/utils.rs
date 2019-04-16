use super::{CompilerData, FunctionType, Scope, Type};
use libc::c_char;
use std::ffi::{CStr, CString};
use llvm_sys::{core::*, prelude::*};

/// convert a string into an llvm compatible literal
pub fn to_ptr(s: &str) -> *const c_char {
    let c_string = CString::new(s.clone()).unwrap();
    c_string.into_raw()
}

pub fn to_string(s: *const c_char) -> String {
    unsafe { String::from(CStr::from_ptr(*&s).to_str().unwrap()) }
}

pub fn to_llvm_type(t: &Type) -> LLVMTypeRef {
    unsafe {
        match t {
            &Type::Bool => LLVMInt1Type(),
            &Type::Bytes => LLVMPointerType(LLVMInt8Type(), 0),
            &Type::Byte => LLVMInt8Type(),
            &Type::FunctionPrototype => LLVMVoidType(),
            &Type::Int => LLVMInt64Type(),
            &Type::None => LLVMVoidType(),
            &Type::String => LLVMPointerType(LLVMInt8Type(), 0),
            &Type::Map(ref k, ref v) => LLVMPointerType(LLVMVoidType(), 0),
        }
    }
}
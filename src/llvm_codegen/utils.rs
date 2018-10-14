use super::{CompilerData, FunctionType, Scope, Type};
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

/// Adds a function to both the scope object, and the
/// compiler object. This ensures that the function can be found
/// in the desired scope, and be compiled later on by llvm.
pub fn add_function(
    compiler: &mut CompilerData,
    scope: &mut Scope,
    name: &str,
    function: FunctionType,
) {
    scope.add_function(name, &function.arg_types(), function.name().to_string());
    // next, we add the function to the compiler.
    compiler
        .functions
        .insert(function.name().to_string(), function);
}

// Retrieve a function from the scope, if it exists. Then retrieve the actual
// function from the compiler.
pub fn get_function(
    compiler: &CompilerData,
    scope: &Scope,
    name: &str,
    arg_types: &[Type],
) -> Option<FunctionType> {
    let maybe_function_name = scope.get_function(name, arg_types);
    if let Some(llvm_name) = maybe_function_name {
        if let Some(func) = compiler.functions.get(&llvm_name) {
            return Some(func.clone());
        }
    }
    None
}

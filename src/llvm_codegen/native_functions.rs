/// Native functions that are available as functions within disp.
/// Functions within this module must be publicly exported in the main.rs
/// file, or else LLVM will be unable to discover the externs.
use super::{to_ptr, Context, Function, Type};
use libc::c_char;
use llvm_sys::core::*;
use std::{collections::HashMap, ffi::CStr, io, mem::forget};

// add native functions to a module context, to ensure
// these builtins are available.
pub fn add_native_functions(context: &mut Context) {
    add_function(context, "print", Type::None, &vec![Type::Int], "print");
    add_function(context, "print", Type::None, &vec![Type::Bool], "print");
    add_function(
        context,
        "print",
        Type::None,
        &vec![Type::String],
        "print_string",
    );
    add_function(
        context,
        "print",
        Type::None,
        &vec![Type::Map(Box::new(Type::String), Box::new(Type::Int))],
        "print_map",
    );
    add_function(
        context,
        "println",
        Type::None,
        &vec![Type::Int],
        "print_string",
    );
    add_function(context, "read-line", Type::String, &vec![], "read_line");
    add_function(
        context,
        "map",
        Type::Map(Box::new(Type::String), Box::new(Type::Int)),
        &vec![],
        "create_map",
    );
    add_function(
        context,
        "add",
        Type::None,
        &vec![
            Type::Map(Box::new(Type::String), Box::new(Type::Int)),
            Type::String,
            Type::Bool,
        ],
        "add_to_map",
    );
}

/// a convenience method to add a function to a
/// context
fn add_function(
    context: &mut Context,
    disp_name: &str,
    return_type: Type,
    arg_types: &[Type],
    ffi_name: &str,
) {
    let mut llvm_args = Vec::with_capacity(arg_types.len());
    for arg in arg_types {
        llvm_args.push(arg.to_llvm_type());
    }
    let llvm_function = unsafe {
        let function = LLVMGetNamedFunction(context.module, to_ptr(ffi_name));
        if !function.is_null() {
            function
        } else {
            LLVMAddFunction(
                context.module,
                to_ptr(ffi_name),
                LLVMFunctionType(
                    return_type.to_llvm_type(),
                    llvm_args.as_mut_ptr(),
                    llvm_args.len() as u32,
                    0,
                ),
            )
        }
    };
    context.scope.add_function(
        disp_name,
        Function {
            arg_types: arg_types.to_owned(),
            return_type: return_type,
            function: llvm_function,
        },
    );
}

// no_mangle is required, to ensure that
// it resolves the name that's specified by the method
// signature.
#[no_mangle]
pub extern "C" fn print(value: i64) {
    print!("{}", value);
}

#[no_mangle]
pub extern "C" fn print_string(value: *const c_char) {
    print!("{}", unsafe { CStr::from_ptr(value).to_str().unwrap() });
}

#[no_mangle]
pub extern "C" fn println(value: i64) {
    println!("{}", value);
}

#[no_mangle]
pub extern "C" fn read_line() -> *const c_char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    to_ptr(&input)
}

#[no_mangle]
pub extern "C" fn create_map() -> *mut HashMap<*const c_char, bool> {
    let map = Box::new(HashMap::new());
    Box::into_raw(map)
}

#[no_mangle]
pub extern "C" fn add_to_map(
    map: *mut HashMap<*const c_char, bool>,
    key: *const c_char,
    value: bool,
) {
    let mut map_unpacked = unsafe { Box::from_raw(map) };
    map_unpacked.insert(key, value);
}

#[no_mangle]
pub extern "C" fn print_map(map: *mut HashMap<*const c_char, bool>) {
    let map_unpacked = unsafe { Box::from_raw(map) };
    println!("called print_map");
    // the pointer must be returned back into the general pool,
    // by calling into raw.
    print!("{{");
    for (k, v) in &*map_unpacked {
        print!(
            "{:?}: {}, ",
            unsafe { CStr::from_ptr(*k).to_str().unwrap() },
            v
        );
    }
    print!("}}");
    // forget must be called for things consumed from
    // raw pointers, otherwise rust will attempt to deallocate
    forget(map_unpacked);
}

/// Native functions that are available as functions within disp.
/// Functions within this module must be publicly exported in the main.rs
/// file, or else LLVM will be unable to discover the externs.
use super::{
    add_function, get_function, to_ptr, to_string, Compiler, Function, FunctionType,
    NativeFunction, Type,
};
use libc::c_char;
use llvm_sys::core::*;
use std::{collections::HashMap, ffi::CStr, io, mem::forget};

// add native functions to a module context, to ensure
// these builtins are available.
pub fn add_native_functions(compiler: &mut Compiler) {
    add_function_to_compiler(compiler, "print", Type::None, &vec![Type::Int], "print");
    add_function_to_compiler(
        compiler,
        "print",
        Type::None,
        &vec![Type::Bool],
        "print_bool",
    );
    add_function_to_compiler(
        compiler,
        "print",
        Type::None,
        &vec![Type::String],
        "print_string",
    );
    add_function_to_compiler(
        compiler,
        "print",
        Type::None,
        &vec![Type::Map(Box::new(Type::String), Box::new(Type::Int))],
        "print_map",
    );
    add_function_to_compiler(compiler, "println", Type::None, &vec![Type::Int], "println");
    add_function_to_compiler(
        compiler,
        "println",
        Type::None,
        &vec![Type::String],
        "println_string",
    );
    add_function_to_compiler(compiler, "read-line", Type::String, &vec![], "read_line");
    add_function_to_compiler(
        compiler,
        "map",
        Type::Map(Box::new(Type::String), Box::new(Type::Int)),
        &vec![],
        "create_map",
    );
    add_function_to_compiler(
        compiler,
        "add",
        Type::None,
        &vec![
            Type::Map(Box::new(Type::String), Box::new(Type::Int)),
            Type::String,
            Type::Bool,
        ],
        "add_to_map",
    );
    add_function_to_compiler(
        compiler,
        "count",
        Type::Int,
        &vec![Type::Map(Box::new(Type::String), Box::new(Type::Int))],
        "count_map",
    );
    add_function_to_compiler(
        compiler,
        "Int",
        Type::Int,
        &vec![Type::String],
        "string_to_int",
    );
}

/// a convenience method to add a function to a
/// context
fn add_function_to_compiler(
    compiler: &mut Compiler,
    name: &str,
    return_type: Type,
    arg_types: &[Type],
    ffi_name: &str,
) {
    let mut llvm_args = Vec::with_capacity(arg_types.len());
    for arg in arg_types {
        llvm_args.push(arg.to_llvm_type());
    }
    if let None = compiler.scope.get_function(name, arg_types) {
        add_function(
            &mut compiler.data,
            &mut compiler.scope,
            name,
            FunctionType::Native(NativeFunction {
                name: ffi_name.to_owned(),
                arg_types: arg_types.to_owned(),
                return_type: return_type,
            }),
        );
    }
}

// no_mangle is required, to ensure that
// it resolves the name that's specified by the method
// signature.
#[no_mangle]
pub extern "C" fn print(value: i64) {
    print!("{}", value);
}

#[no_mangle]
pub extern "C" fn print_bool(value: bool) {
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
pub extern "C" fn println_string(value: *const c_char) {
    println!("{}", unsafe { CStr::from_ptr(value).to_str().unwrap() });
}

#[no_mangle]
pub extern "C" fn read_line() -> *const c_char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.pop(); // remove newline
    to_ptr(&input)
}

#[no_mangle]
pub extern "C" fn create_map() -> *mut HashMap<String, bool> {
    let map = Box::new(HashMap::new());
    Box::into_raw(map)
}

#[no_mangle]
pub extern "C" fn add_to_map(map: *mut HashMap<String, bool>, key: *const c_char, value: bool) {
    let map_unpacked = unsafe { &mut *map };
    map_unpacked.insert(to_string(key), value);
    forget(map_unpacked);
}

#[no_mangle]
pub extern "C" fn count_map(map: *mut HashMap<String, bool>) -> i64 {
    let map_unpacked = unsafe { &*map };
    let len = map_unpacked.len() as i64;
    len
}

#[no_mangle]
pub extern "C" fn print_map(map: *mut HashMap<String, bool>) {
    let map_unpacked = unsafe { &*map };
    // the pointer must be returned back into the general pool,
    // by calling into raw.
    print!("{{");
    for (k, v) in &*map_unpacked {
        print!("{}: {}, ", k, v);
    }
    print!("}}");
}

#[no_mangle]
pub extern "C" fn string_to_int(s: *const c_char) -> i64 {
    let int_as_string = unsafe { CStr::from_ptr(s).to_str().unwrap() };
    match int_as_string.parse::<i64>() {
        Ok(i) => i,
        Err(_) => panic!(format!("string {} is not an integer", int_as_string)),
    }
}

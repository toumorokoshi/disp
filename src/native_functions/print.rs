use super::Compiler;

pub fn add_print(compiler: &mut Compiler) {
    add_function_to_compiler(compiler, "print", Type::None, &vec![Type::Int], "print");
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
pub extern "C" fn print_string(value: *const c_char) {
    print!("{}", unsafe { CStr::from_ptr(value).to_str().unwrap() });
}

#[no_mangle]
pub extern "C" fn print_bool(value: bool) {
    print!("{}", value);
}

#[no_mangle]
pub extern "C" fn print_int(value: i64) {
    print!("{}", value);
}

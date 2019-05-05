use super::*;

pub fn expression() -> Expression {
    Expression {
        boostrap_compiler: boostrap_compiler,
        typecheck: typecheck,
        codegen: codegen,
    }
}

fn boostrap_compiler(compiler: &mut Compiler) {
    add_function_to_compiler(compiler, "print", Type::None, &vec![Type::Int], "print_int");
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
        &vec![Type::Array(Box::new(Type::Byte))],
        "print_bytes",
    );
    add_function_to_compiler(
        compiler,
        "print",
        Type::None,
        &vec![Type::Map(Box::new(Type::String), Box::new(Type::Int))],
        "print_map",
    );
    add_function_to_compiler(
        compiler,
        "print",
        Type::None,
        &vec![Type::Byte],
        "print_byte",
    );
}

fn typecheck(
    resolver: &mut TypeResolver<Type>,
    _function: &TypevarFunction,
    _args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    let type_var = resolver.create_type_var();
    resolver.add_constraint(Constraint::IsLiteral(type_var, Type::None))?;
    Ok(type_var)
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    call_function(context, "print", args)
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

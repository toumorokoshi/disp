use super::*;
use std::{
    ffi::{CStr, CString},
    io,
};

pub fn expression() -> Expression {
    Expression {
        boostrap_compiler: boostrap_compiler,
        typecheck: typecheck,
        codegen: codegen,
    }
}

fn boostrap_compiler(compiler: &mut Compiler) {
    add_function_to_compiler(compiler, "read-line", Type::String, &vec![], "readline");
}

fn typecheck(
    resolver: &mut TypeResolver<Type>,
    _function: &TypevarFunction,
    _args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    let type_var = resolver.create_type_var();
    Ok(type_var)
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    call_function(context, "read-line", args)
}

#[no_mangle]
pub extern "C" fn readline() -> *const c_char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let c_str = CString::new(input).unwrap();
    c_str.into_raw()
}

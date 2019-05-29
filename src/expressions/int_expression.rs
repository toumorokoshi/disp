use super::*;
use std::ffi::CStr;

pub fn expression() -> Expression {
    Expression {
        boostrap_compiler: boostrap_compiler,
        typecheck: typecheck,
        codegen: codegen,
    }
}

fn boostrap_compiler(compiler: &mut Compiler) {
    add_function_to_compiler(compiler, "int", Type::Int, &vec![Type::String], "int");
}

fn typecheck(
    resolver: &mut TypeResolver<TypecheckType>,
    _function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    let type_var = resolver.create_type_var();
    resolver.add_constraint(Constraint::IsLiteral(
        args[0],
        Unresolved::Literal(TypecheckType::String),
    ))?;
    resolver.add_constraint(Constraint::IsLiteral(
        type_var,
        Unresolved::Literal(TypecheckType::Int),
    ))?;
    Ok(type_var)
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    call_function(context, "int", args)
}

#[no_mangle]
pub extern "C" fn int(value: *const c_char) -> i64 {
    let s = unsafe { CStr::from_ptr(value).to_str().unwrap() };
    s.parse::<i64>().unwrap()
}

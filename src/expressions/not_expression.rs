use super::*;

pub fn expression() -> Expression {
    Expression {
        boostrap_compiler: boostrap_compiler,
        typecheck: typecheck,
        codegen: codegen,
    }
}

fn boostrap_compiler(compiler: &mut Compiler) {
    add_function_to_compiler(
        compiler,
        "not",
        Type::Bool,
        &vec![Type::Bool],
        "not",
    );
}

fn typecheck(
    resolver: &mut TypeResolver<Type>,
    _function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    let type_var = resolver.create_type_var();
    resolver.add_constraint(Constraint::IsLiteral(type_var, Type::Bool))?;
    resolver.add_constraint(Constraint::IsLiteral(args[0], Type::Bool))?;
    Ok(type_var)
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    call_function(context, "not", args)
}


#[no_mangle]
pub extern "C" fn not(value: bool) -> bool {
    !value
}
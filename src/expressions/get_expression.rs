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
        "get",
        Type::Byte,
        &vec![Type::Bytes, Type::Int],
        "get_bytes",
    );
}

fn typecheck(
    resolver: &mut TypeResolver<Type>,
    _function: &TypevarFunction,
    _args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    let type_var = resolver.create_type_var();
    resolver.add_constraint(Constraint::IsLiteral(type_var, Type::Byte))?;
    Ok(type_var)
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    call_function(context, "get", args)
}

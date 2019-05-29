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
        "len",
        Type::Int,
        &vec![Type::Array(Box::new(Type::Byte))],
        "len_bytes",
    );
}

fn typecheck(
    resolver: &mut TypeResolver<TypecheckType>,
    _function: &TypevarFunction,
    _args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    let type_var = resolver.create_type_var();
    resolver.add_constraint(Constraint::IsLiteral(type_var, 
        Unresolved::Literal(TypecheckType::Int)))?;
    Ok(type_var)
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    call_function(context, "len", args)
}

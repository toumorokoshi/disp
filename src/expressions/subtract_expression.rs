use super::*;

pub fn expression() -> Expression {
    Expression {
        typecheck: typecheck,
        codegen: codegen,
    }
}

fn typecheck(
    resolver: &mut TypeResolver<Type>,
    function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    // TODO: figure out how to recurse into nested
    // data structure type variables.
    resolver.add_constraint(Constraint::Equality(args[1].clone(), args[0].clone()))?;
    resolver.add_constraint(Constraint::IsLiteral(args[0].clone(), Type::Int));
    Ok(args[0].clone())
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    codegen_binop(context, args, LLVMOpcode::LLVMSub)
}

use super::*;

pub fn expression() -> Expression {
    Expression {
        boostrap_compiler: boostrap_compiler,
        typecheck: typecheck,
        codegen: codegen,
    }
}

fn boostrap_compiler(_compiler: &mut Compiler) {}

fn typecheck(
    resolver: &mut TypeResolver<TypecheckType>,
    _function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    let type_var = resolver.create_type_var();
    resolver.add_constraint(Constraint::IsLiteral(
        type_var,
        Unresolved::Literal(TypecheckType::Bool),
    ))?;
    resolver.add_constraint(Constraint::IsLiteral(
        args[0],
        Unresolved::Literal(TypecheckType::Bool),
    ))?;
    Ok(type_var)
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    let source = gen_token(context, &args[0])?;
    let target = context.allocate(Type::Bool);
    context.add_instruction(LLVMInstruction::BuildNot {
        source: source.index,
        target: target.index,
    });
    Ok(target)
}

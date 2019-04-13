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
    resolver: &mut TypeResolver<Type>,
    _function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    let type_var = resolver.create_type_var();
    resolver.add_constraint(Constraint::Equality(args[0], args[1]))?;
    resolver.add_constraint(Constraint::IsLiteral(type_var, Type::Bool))?;
    Ok(type_var)
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    if args.len() != 2 {
        return Err(CodegenError::new(&format!(
            "eq expression should only have two arguments. found {}",
            args.len()
        )));
    };
    let lhs = gen_token(context, &args[0])?;
    let rhs = gen_token(context, &args[1])?;
    let result = context.allocate(Type::Bool);
    context.add_instruction(LLVMInstruction::BuildICmp {
        lhs: lhs.index,
        rhs: rhs.index,
        target: result.index,
    });
    Ok(result)
}

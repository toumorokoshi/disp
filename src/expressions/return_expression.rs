use super::*;

pub fn expression() -> Expression {
    Expression {
        boostrap_compiler,
        typecheck,
        codegen,
    }
}

fn boostrap_compiler(_compiler: &mut Compiler) {}

pub fn typecheck(
    resolver: &mut TypeResolver<TypecheckType>,
    function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    resolver.add_constraint(Constraint::Equality(
        function.return_type.clone(),
        args[0].clone(),
    ))?;
    Ok(args[0].clone())
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    if args.len() != 1 {
        return Err(CodegenError::new(&format!(
            "expected one argument for return, found {}: {:?}",
            args.len(),
            args
        )));
    }
    let result = gen_token(context, &args[0])?;
    context.add_instruction(LLVMInstruction::BuildRet {
        source: result.index,
    });
    Ok(result)
}

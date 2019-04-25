use super::*;

pub fn expression() -> Expression {
    Expression {
        boostrap_compiler: boostrap_compiler,
        typecheck: typecheck,
        codegen: codegen,
    }
}

fn boostrap_compiler(compiler: &mut Compiler) {}

fn typecheck(
    resolver: &mut TypeResolver<Type>,
    _function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    let type_var = resolver.create_type_var();
    resolver.add_constraint(Constraint::IsLiteral(type_var, Type::None))?;
    resolver.add_constraint(Constraint::IsLiteral(args[0], Type::None))?;
    Ok(type_var)
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    if args.len() != 2 {
        return Err(CodegenError::new(&format!(
            "while expression should only have two arguments (conditional and body). found {}",
            args.len()
        )));
    };
    // insert a basic block loop, to jump back to.
    let condition_block = context.create_block("loop_condition".to_owned());
    let loop_block = context.create_block("loop".to_owned());
    let after_loop_block = context.create_block("after_loop".to_owned());
    // go immediately to the block
    context.add_instruction(LLVMInstruction::BuildBr {
        block: condition_block,
    });
    {
        let mut branch_context = Context::new(
            context.function_map,
            context.compiler,
            context.function,
            context.scope,
            condition_block,
        );
        let condition_result = gen_token(&mut branch_context, &args[0])?;
        branch_context.add_instruction(LLVMInstruction::BuildCondBr {
            value: condition_result.index,
            true_block: loop_block,
            false_block: after_loop_block,
        });
    }
    let loop_result = {
        let mut loop_context = Context::new(
            context.function_map,
            context.compiler,
            context.function,
            context.scope,
            loop_block,
        );
        let loop_result = gen_token(&mut loop_context, &args[1])?;
        loop_context.add_instruction(LLVMInstruction::BuildBr {
            block: condition_block,
        });
        loop_result
    };
    context.block = after_loop_block;
    Ok(loop_result)
}

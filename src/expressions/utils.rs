use super::*;

pub fn codegen_binop(
    context: &mut Context,
    args: &[Token],
    op: LLVMOpcode,
) -> CodegenResult<Object> {
    if args.len() != 2 {
        return Err(CodegenError::new(&format!(
            "binary expression should only have two arguments. found {}",
            args.len()
        )));
    };
    let lhs = gen_token(context, &args[0])?;
    let rhs = gen_token(context, &args[1])?;
    let result = context.allocate(Type::Int);
    context.add_instruction(LLVMInstruction::BuildBinOp {
        opcode: op,
        lhs: lhs.index,
        rhs: rhs.index,
        target: result.index,
    });
    Ok(result)
}

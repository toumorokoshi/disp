use super::{gen_token, to_ptr, CodegenError, CodegenResult, Context, Object, Token, Type};
use llvm_sys::{core::*, *};
pub fn let_production<'a, 'b>(
    context: &'a mut Context<'b>,
    args: &[Token],
) -> CodegenResult<Object> {
    if args.len() != 2 {
        return Err(CodegenError::new(&format!(
            "let function should only have two arguments. found {}",
            args.len()
        )));
    };
    let var_name = match &args[0] {
        Token::Symbol(ref s) => s.clone(),
        t => {
            return Err(CodegenError::new(&format!(
                "expected a symbol for the first argument. found {}",
                t,
            )));
        }
    };
    let target = gen_token(context, &args[1])?;
    context
        .scope
        .locals
        .insert(*var_name.clone(), target.clone());
    Ok(target)
}

pub fn equals_production<'a, 'b>(
    context: &'a mut Context<'b>,
    args: &[Token],
) -> CodegenResult<Object> {
    if args.len() != 2 {
        return Err(CodegenError::new(&format!(
            "equals function should only have two arguments. found {}",
            args.len()
        )));
    };
    let lhs = gen_token(context, &args[0])?;
    let rhs = gen_token(context, &args[1])?;
    let result = unsafe {
        LLVMBuildICmp(
            context.builder,
            LLVMIntPredicate::LLVMIntEQ,
            lhs.value,
            rhs.value,
            to_ptr("eqtemp"),
        )
    };
    Ok(Object::new(result, Type::Bool))
}

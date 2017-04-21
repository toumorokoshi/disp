use self::{Context, CodegenResult, Object, gen_token};

pub type Production = fn(context: &mut Context, args: &[Token]) -> CodegenResult;

pub fn plus_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let lhs = ensure_int!(gen_token(args[0]));
    let rhs = ensure_int!(gen_token(args[1]));
    let obj = context.builder.allocate_local(ghvm::Type::Int);
    context.builder.ops.push(ghvm::Op::IntAdd{
        lhs: lhs.register, rhs: rhs.register, target: obj.register;
    });
    return Ok(Object::from_build_object(obj));
}

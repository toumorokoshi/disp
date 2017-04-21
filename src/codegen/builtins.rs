use ghvm;
use super::{Context, CodegenResult, Object, gen_token};
use super::super::{Token};

macro_rules! ensure_type {
    ($x:pat, $y:expr) => {
        match $y.typ {
            $x => Ok($y),
            _ => Err(String::from("type did not match"))
        }
    }
}

pub type Production = fn(context: &mut Context, args: &[Token]) -> CodegenResult;

pub fn plus_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let lhs = try!(ensure_type!(ghvm::Type::Int, try!(gen_token(context, &args[0]))));
    let rhs = try!(ensure_type!(ghvm::Type::Int, try!(gen_token(context, &args[1]))));
    let obj = context.builder.allocate_local(ghvm::Type::Int);
    context.builder.ops.push(ghvm::Op::IntAdd{
        lhs: lhs.register, rhs: rhs.register, target: obj.register
    });
    return Ok(Object::from_build_object(obj));
}

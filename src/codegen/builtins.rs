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
    let obj = context.builder.allocate_local(&ghvm::Type::Int);
    context.builder.ops.push(ghvm::Op::IntAdd{
        lhs: lhs.register, rhs: rhs.register, target: obj.register
    });
    return Ok(Object::from_build_object(obj));
}

pub fn equals_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let lhs = try!(ensure_type!(ghvm::Type::Int, try!(gen_token(context, &args[0]))));
    let rhs = try!(ensure_type!(ghvm::Type::Int, try!(gen_token(context, &args[1]))));
    let obj = context.builder.allocate_local(&ghvm::Type::Int);
    context.builder.ops.push(ghvm::Op::IntCmp{
        lhs: lhs.register, rhs: rhs.register, target: obj.register
    });
    return Ok(Object::from_build_object(obj));
}

pub fn not_equals_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let lhs = try!(ensure_type!(ghvm::Type::Int, try!(gen_token(context, &args[0]))));
    let rhs = try!(ensure_type!(ghvm::Type::Int, try!(gen_token(context, &args[1]))));
    let obj = context.builder.allocate_local(&ghvm::Type::Int);
    context.builder.ops.push(ghvm::Op::IntCmp{
        lhs: lhs.register, rhs: rhs.register, target: obj.register
    });
    context.builder.ops.push(ghvm::Op::BoolNot{
        source: obj.register, target: obj.register
    });
    return Ok(Object::from_build_object(obj));
}

// TODO: this should return a join algebraic type
// of both blocks in the case that they are different,
// and return a single type if they are the same.
// but support for algebraic types need to be added first.
pub fn if_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let condition = try!(ensure_type!(ghvm::Type::Bool, try!(gen_token(context, &args[0]))));
    // TODO: support more than int
    let return_value = context.builder.allocate_local(&ghvm::Type::Int);
    let branch_index = context.builder.ops.len();
    // placeholder to replace with branch
    context.builder.ops.push(ghvm::Op::Noop{});
    // if true block
    let true_result = try!(gen_token(context, &args[1]));
    context.builder.ops.push(ghvm::Op::Assign{source: true_result.register, target: return_value.register});
    // placeholder for jump to the end.
    let goto_index = context.builder.ops.len();
    context.builder.ops.push(ghvm::Op::Noop{});
    // false block
    let false_index = context.builder.ops.len();
    let false_result = try!(gen_token(context, &args[2]));
    context.builder.ops.push(ghvm::Op::Assign{source: false_result.register, target: return_value.register});
    context.builder.ops[branch_index] = ghvm::Op::Branch{condition: condition.register, if_false: false_index};
    context.builder.ops[goto_index] = ghvm::Op::Goto{position: context.builder.ops.len()};
    return Ok(Object{typ: ghvm::Type::Int, register: return_value.register});
}

pub fn while_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let start_index = context.builder.ops.len();
    let return_value = context.builder.allocate_local(&ghvm::Type::Int);
    let condition = try!(ensure_type!(ghvm::Type::Bool, try!(gen_token(context, &args[0]))));
    // placeholder for the condition check
    context.builder.ops.push(ghvm::Op::Noop{});
    let branch_index = context.builder.ops.len();
    let return_value = try!(gen_token(context, &args[1]));
    context.builder.ops.push(ghvm::Op::Goto{position: start_index});
    let loop_end_index = context.builder.ops.len();
    context.builder.ops[branch_index] = ghvm::Op::Branch{condition: condition.register, if_false: context.builder.ops.len()};
    return Ok(Object{typ: ghvm::Type::Int, register: return_value.register});
}

pub fn mut_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    match args[0] {
        Token::Symbol(ref s) => {
            let source = try!(gen_token(context, &args[1]));
            let target = Object::from_build_object(context.builder.get_insert_local_var(&source.typ, s));
            if source.typ != target.typ {
                Err(String::from("mut expression type collision"))
            } else {
                context.builder.ops.push(
                    ghvm::Op::Assign{target: target.register, source: source.register}
                );
                Ok(target)
            }
        },
        _ => Err(String::from("first value must be a symbol."))
    }
}


pub fn print(vm: &mut ghvm::VM, mut args: ghvm::ValueList) -> ghvm::Value {
    println!("{0}", args[0]);
    0
}

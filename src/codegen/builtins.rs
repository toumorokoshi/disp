use ghvm;
use super::{Context, CodegenResult, Object, gen_token};
use super::super::{Token, HashableToken};

macro_rules! ensure_type {
    ($x:pat, $y:expr) => {
        match $y.typ {
            $x => Ok($y),
            _ => Err(format!("type did not match. found {}", $y.typ))
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
    let obj = context.builder.allocate_local(&ghvm::Type::Bool);
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
    context.builder.ops[branch_index] = ghvm::Op::BranchFalse{condition: condition.register, if_false: false_index};
    context.builder.ops[goto_index] = ghvm::Op::Goto{position: context.builder.ops.len()};
    return Ok(Object{typ: ghvm::Type::Int, register: return_value.register});
}

pub fn while_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let start_index = context.builder.ops.len();
    let return_value = context.builder.allocate_local(&ghvm::Type::Int);
    let condition = try!(ensure_type!(ghvm::Type::Bool, try!(gen_token(context, &args[0]))));
    // placeholder for the condition check
    let branch_index = context.builder.ops.len();
    context.builder.ops.push(ghvm::Op::Noop{});
    let return_value = try!(gen_token(context, &args[1]));
    context.builder.ops.push(ghvm::Op::Goto{position: start_index});
    let loop_end_index = context.builder.ops.len();
    context.builder.ops[branch_index] = ghvm::Op::BranchFalse{condition: condition.register, if_false: context.builder.ops.len()};
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


pub fn match_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let var_to_match = try!(gen_token(context, &args[0]));
    match &args[1] {
        &Token::Dict(ref d) => {
            let condition_temp = context.builder.allocate_local(&ghvm::Type::Bool);
            let result = context.builder.allocate_local(&ghvm::Type::Int);
            let pairs: Vec<(&HashableToken, &Token)> = d.iter().collect();

            // first, we build the key objects
            let mut key_objects = vec![];
            for pair in &pairs {
                let resolved_key_token = (*pair).0.as_token();
                let key = try!(gen_token(context, &resolved_key_token));
                key_objects.push(key);
            }

            let head_index = context.builder.ops.len();
            // then, we create empty registers to replace with branching
            for i in 0..key_objects.len() {
                // we need two ops: an IntCmp and a BranchTrue afterward
                context.builder.ops.push(ghvm::Op::Noop{});
                context.builder.ops.push(ghvm::Op::Noop{});
            }
            // one last op to replace with a goto if nothing matches.
            let final_goto_index = context.builder.ops.len();
            context.builder.ops.push(ghvm::Op::Noop{});

            // finally, we build the bodies, and replace the noops with
            // branches
            for (index, pair) in pairs.iter().enumerate() {
                let key_object = &key_objects[index];
                context.builder.ops[head_index + index * 2] = ghvm::Op::IntCmp{
                    lhs: var_to_match.register,
                    rhs: key_object.register,
                    target: condition_temp.register
                };
                context.builder.ops[head_index + index * 2 + 1] = ghvm::Op::BranchTrue{
                    condition: condition_temp.register, if_true: context.builder.ops.len()
                };
                let block_result = try!(gen_token(context, (*pair).1));
                context.builder.ops.push(ghvm::Op::Assign{
                    source: block_result.register, target: result.register,
                });
            }
            context.builder.ops[final_goto_index] = ghvm::Op::Goto{
                position: context.builder.ops.len() - 1
            };
            // TODO: replace none with the successful result.
            Ok(Object::from_build_object(result))
        },
        _ => Err(format!("second argument to a match should be a dict"))
    }
}


pub fn print(vm: &mut ghvm::VM, mut args: ghvm::ValueList) -> ghvm::Value {
    println!("{0}", args[0]);
    0
}

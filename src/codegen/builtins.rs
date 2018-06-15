use std::sync::Arc;
use warpspeed::{Op, Type};
use super::{compile, Context, CodegenResult, Object, gen_token};
use super::super::{Token, HashableToken};

macro_rules! ensure_type {
    ($x:pat, $y:expr) => {
        match $y.typ {
            $x => Ok($y),
            _ => Err(format!("type did not match. found {}", $y.typ))
        }
    }
}

// pub type Production = fn(context: &mut Context, args: &[Token]) -> CodegenResult;

pub fn plus_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let lhs = try!(ensure_type!(Type::Int, try!(gen_token(context, &args[0]))));
    let rhs = try!(ensure_type!(Type::Int, try!(gen_token(context, &args[1]))));
    let obj = context.builder.allocate_local(&Type::Int);
    context.builder.ops.push(Op::IntAdd{
        lhs: lhs.register, rhs: rhs.register, target: obj.register
    });
    return Ok(Object::from_build_object(obj));
}

pub fn minus_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let lhs = try!(ensure_type!(Type::Int, try!(gen_token(context, &args[0]))));
    let rhs = try!(ensure_type!(Type::Int, try!(gen_token(context, &args[1]))));
    let obj = context.builder.allocate_local(&Type::Int);
    context.builder.ops.push(Op::IntSub{
        lhs: lhs.register, rhs: rhs.register, target: obj.register
    });
    return Ok(Object::from_build_object(obj));
}


pub fn equals_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let lhs = try!(ensure_type!(Type::Int, try!(gen_token(context, &args[0]))));
    let rhs = try!(ensure_type!(Type::Int, try!(gen_token(context, &args[1]))));
    let obj = context.builder.allocate_local(&Type::Int);
    context.builder.ops.push(Op::IntCmp{
        lhs: lhs.register, rhs: rhs.register, target: obj.register
    });
    return Ok(Object::from_build_object(obj));
}

pub fn not_equals_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let lhs = try!(ensure_type!(Type::Int, try!(gen_token(context, &args[0]))));
    let rhs = try!(ensure_type!(Type::Int, try!(gen_token(context, &args[1]))));
    let obj = context.builder.allocate_local(&Type::Bool);
    context.builder.ops.push(Op::IntCmp{
        lhs: lhs.register, rhs: rhs.register, target: obj.register
    });
    context.builder.ops.push(Op::BoolNot{
        source: obj.register, target: obj.register
    });
    return Ok(Object::from_build_object(obj));
}

// TODO: this should return a join algebraic type
// of both blocks in the case that they are different,
// and return a single type if they are the same.
// but support for algebraic types need to be added first.
pub fn if_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let condition = try!(ensure_type!(Type::Bool, try!(gen_token(context, &args[0]))));
    // TODO: support more than int
    let return_value = context.builder.allocate_local(&Type::Int);
    let branch_index = context.builder.ops.len();
    // placeholder to replace with branch
    context.builder.ops.push(Op::Noop{});
    // if true block
    let true_result = try!(gen_token(context, &args[1]));
    context.builder.ops.push(Op::Assign{source: true_result.register, target: return_value.register});
    // placeholder for jump to the end.
    let goto_index = context.builder.ops.len();
    context.builder.ops.push(Op::Noop{});
    // false block
    let false_index = context.builder.ops.len();
    let false_result = try!(gen_token(context, &args[2]));
    context.builder.ops.push(Op::Assign{source: false_result.register, target: return_value.register});
    context.builder.ops[branch_index] = Op::BranchFalse{condition: condition.register, if_false: false_index};
    context.builder.ops[goto_index] = Op::Goto{position: context.builder.ops.len()};
    return Ok(Object{typ: Type::Int, register: return_value.register});
}

pub fn while_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let start_index = context.builder.ops.len();
    let condition = try!(ensure_type!(Type::Bool, try!(gen_token(context, &args[0]))));
    // placeholder for the condition check
    let branch_index = context.builder.ops.len();
    context.builder.ops.push(Op::Noop{});
    let return_value = try!(gen_token(context, &args[1]));
    context.builder.ops.push(Op::Goto{position: start_index});
    // let loop_end_index = context.builder.ops.len();
    context.builder.ops[branch_index] = Op::BranchFalse{condition: condition.register, if_false: context.builder.ops.len()};
    return Ok(Object{typ: Type::Int, register: return_value.register});
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
                    Op::Assign{target: target.register, source: source.register}
                );
                Ok(target)
            }
        },
        _ => Err(String::from("first value must be a symbol."))
    }
}

pub fn const_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    match args[0] {
        Token::Symbol(ref s) => {
            match context.builder.get_var(s) {
                Some(_) => {
                    Err(format!("cannot declare const variable {} twice", s))
                },
                None => {
                    let source = try!(gen_token(context, &args[1]));
                    let target = Object::from_build_object(context.builder.get_insert_local_var(&source.typ, s));
                    context.builder.ops.push(
                        Op::Assign{target: target.register, source: source.register}
                    );
                    Ok(target)
                }
            }
        },
        _ => Err(String::from("first value must be a symbol."))
    }
}

pub fn function_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    // the first argument is a list of variables, so we pull those.
    // TODO: parse into VMFunction declaration.
    let variables = try!(gen_token(context, &args[0]));
    let function = compile(&mut context.vm, &args[1]).unwrap();
    // add the function to the VM, so it can be referenced in bytecode.
    match Arc::get_mut(&mut context.vm.heap) {
        None => Err(String::from("unable to get add a method to the vm (unable to get a heap handle)")),
        Some(heap) => {
            heap.functions_vm.push(Arc::new(function));
            let function_index = heap.functions_vm.len();
            let function_register = context.builder.allocate_local(&Type::FunctionVM);
            context.builder.ops.push(Op::FunctionVMLoad{
                func_index: function_index,
                target: function_register.register,
            });
            Ok(Object::from_build_object(function_register))
        }
    }
}

pub fn match_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let var_to_match = try!(gen_token(context, &args[0]));
    match &args[1] {
        &Token::Dict(ref d) => {
            let condition_temp = context.builder.allocate_local(&Type::Bool);
            let result = context.builder.allocate_local(&Type::Int);
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
            for _ in 0..key_objects.len() {
                // we need two ops: an IntCmp and a BranchTrue afterward
                context.builder.ops.push(Op::Noop{});
                context.builder.ops.push(Op::Noop{});
            }
            // one last op to replace with a goto if nothing matches.
            let final_goto_index = context.builder.ops.len();
            context.builder.ops.push(Op::Noop{});

            // finally, we build the bodies, and replace the noops with
            // branches
            for (index, pair) in pairs.iter().enumerate() {
                let key_object = &key_objects[index];
                context.builder.ops[head_index + index * 2] = Op::IntCmp{
                    lhs: var_to_match.register,
                    rhs: key_object.register,
                    target: condition_temp.register
                };
                context.builder.ops[head_index + index * 2 + 1] = Op::BranchTrue{
                    condition: condition_temp.register, if_true: context.builder.ops.len()
                };
                let block_result = try!(gen_token(context, (*pair).1));
                context.builder.ops.push(Op::Assign{
                    source: block_result.register, target: result.register,
                });
            }
            context.builder.ops[final_goto_index] = Op::Goto{
                position: context.builder.ops.len() - 1
            };
            // TODO: replace none with the successful result.
            Ok(Object::from_build_object(result))
        },
        _ => Err(format!("second argument to a match should be a dict"))
    }
}

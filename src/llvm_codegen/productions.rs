use super::{
    gen_token, to_ptr, CodegenError, CodegenResult, Context, FunctionPrototype, LLVMInstruction,
    Object, Token, Type,
};
use llvm_sys::LLVMOpcode;

macro_rules! ensure_type {
    ($x:ident, $y:expr) => {
        if $x.object_type != $y {
            return Err(CodegenError::new(&format!(
                "type did not match. found {:?}",
                $x.object_type
            )));
        }
    };
}

pub fn let_production<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
    args: &[Token],
) -> CodegenResult<Object> {
    if args.len() != 2 {
        return Err(CodegenError::new(&format!(
            "let function should only have two arguments. found {}: {:?}",
            args.len(),
            args
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
    match target.function_prototype {
        Some(ref function) => {
            context
                .scope
                .function_prototypes
                .insert(*var_name.clone(), function.clone());
            Ok(target.clone())
        }
        None => {
            let result_object = context.scope.locals.entry(*var_name.clone()).or_insert({
                let object = context.function.objects;
                context.function.objects += 1;
                context
                    .function
                    .instructions
                    .push(LLVMInstruction::BuildAlloca {
                        llvm_type: target.object_type.to_llvm_type(),
                        target: object,
                    });
                Object::new(object, target.object_type.clone())
            });
            context
                .function
                .instructions
                .push(LLVMInstruction::BuildStore {
                    source: target.index,
                    target: result_object.index,
                });
            Ok(result_object.clone())
        }
    }
}

pub fn equals_production<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
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
    let target = context.allocate(Type::Bool);
    context
        .function
        .instructions
        .push(LLVMInstruction::BuildICmp {
            lhs: lhs.index,
            rhs: rhs.index,
            target: target.index,
        });
    Ok(target)
}

pub fn while_production<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
    args: &[Token],
) -> CodegenResult<Object> {
    if args.len() != 2 {
        return Err(CodegenError::new(&format!(
            "while expression should only have two arguments (conditional and body). found {}",
            args.len()
        )));
    };
    // insert a basic block loop, to jump back to.
    let condition_block = context.add_basic_block("loop_condition".to_owned());
    let loop_block = context.add_basic_block("loop".to_owned());
    let after_loop_block = context.add_basic_block("after_loop".to_owned());
    // go immediately to the block
    context.add_instruction(LLVMInstruction::BuildBr {
        block: condition_block,
    });
    context.add_instruction(LLVMInstruction::PositionBuilderAtEnd {
        block: condition_block,
    });
    context.block = condition_block;
    let condition_result = gen_token(context, &args[0])?;
    context.add_instruction(LLVMInstruction::BuildCondBr {
        value: condition_result.index,
        true_block: loop_block,
        false_block: after_loop_block,
    });
    context.add_instruction(LLVMInstruction::PositionBuilderAtEnd { block: loop_block });
    let result = gen_token(context, &args[1])?;
    // loop back to conditional
    context.add_instruction(LLVMInstruction::BuildBr {
        block: condition_block,
    });
    context.add_instruction(LLVMInstruction::PositionBuilderAtEnd {
        block: after_loop_block,
    });
    Ok(result)
}

pub fn add_production<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
    args: &[Token],
) -> CodegenResult<Object> {
    if args.len() != 2 {
        return Err(CodegenError::new(&format!(
            "add expression should only have two arguments. found {}",
            args.len()
        )));
    };
    let lhs = gen_token(context, &args[0])?;
    ensure_type!(lhs, Type::Int);
    let rhs = gen_token(context, &args[1])?;
    ensure_type!(rhs, Type::Int);
    let result = context.allocate(Type::Int);
    context.add_instruction(LLVMInstruction::BuildBinOp {
        opcode: LLVMOpcode::LLVMAdd,
        lhs: lhs.index,
        rhs: rhs.index,
        target: result.index,
    });
    Ok(result)
}

pub fn not_production<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
    args: &[Token],
) -> CodegenResult<Object> {
    if args.len() != 1 {
        return Err(CodegenError::new(&format!(
            "not expression should only have one argument. found {}",
            args.len()
        )));
    };
    let result_to_negate = gen_token(context, &args[0])?;
    ensure_type!(result_to_negate, Type::Bool);
    let result = context.allocate(Type::Bool);
    context.add_instruction(LLVMInstruction::BuildNot {
        source: result_to_negate.index,
        target: result.index,
    });
    Ok(result)
}

pub fn match_production<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
    args: &[Token],
) -> CodegenResult<Object> {
    if args.len() != 2 {
        return Err(CodegenError::new(&format!(
            "match expression should have two arguments: a value to match, and a map to match against. found {:?}",
            args
        )));
    };
    let condition = gen_token(context, &args[0])?;
    unsafe {
        let post_switch_block = context.add_basic_block("switchcomplete".to_owned());
        if let Token::Map(ref map) = &args[1] {
            let mut key_values = vec![];
            // we construct all keys first, to ensure
            // that they exist before the match statement is
            // executed.
            // TODO: reject keys that are not constants
            for key in map.keys() {
                let key_value = gen_token(context, &key.as_token())?;
                ensure_type!(key_value, condition.object_type);
                key_values.push(key_value);
            }
            let num_cases = (map.len() - 1) as u32;
            let switch = context.allocate_without_type();
            context.add_instruction(LLVMInstruction::BuildSwitch {
                value: condition.index,
                post_switch_block: post_switch_block,
                num_cases: num_cases,
                target: switch,
            });
            for (index, (_key, value)) in map.iter().enumerate() {
                let block = context.add_basic_block("case".to_owned());
                let branch_key = &key_values[index];
                context.add_instruction(LLVMInstruction::AddCase {
                    switch,
                    value: branch_key.index,
                    block,
                });
                context.add_instruction(LLVMInstruction::PositionBuilderAtEnd { block });
                // TODO: capture this value and make it the return value of the
                // match statement.
                gen_token(context, value)?;
                context.add_instruction(LLVMInstruction::BuildBr {
                    block: post_switch_block,
                });
            }
        } else {
            return Err(CodegenError::new(&format!(
                "match expression should be map. found {}",
                &args[1]
            )));
        }
        context.add_instruction(LLVMInstruction::PositionBuilderAtEnd {
            block: post_switch_block,
        });
        context.block = post_switch_block;
    }
    Ok(Object::none())
}

pub fn fn_production<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
    args: &[Token],
) -> CodegenResult<Object> {
    if args.len() != 2 {
        return Err(CodegenError::new(&format!(
            "fn expression should only have two arguments: one for variable names and one for the body. found {}: {:?}",
            args.len(), args
        )));
    };
    let argument_symbols = if let Token::List(ref l) = &args[0] {
        let mut names = vec![];
        for variable_token in l {
            if let Token::Symbol(ref s) = variable_token {
                names.push(*s.clone());
            } else {
                return Err(CodegenError::new(&format!(
                    "argument name must be a symbol. found {}",
                    variable_token
                )));
            }
        }
        names
    } else {
        return Err(CodegenError::new(&format!(
            "second argument to fn should be a list. found {}",
            &args[0]
        )));
    };
    let body = match &args[1] {
        Token::List(ref l) => l.clone(),
        t => {
            return Err(CodegenError::new(&format!(
                "body for function must be a list of tokens. found {}",
                t
            )));
        }
    };
    let prototype = FunctionPrototype {
        argument_symbols: argument_symbols,
        body: body,
    };
    Ok(Object::function_prototype(prototype))
}

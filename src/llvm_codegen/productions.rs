use super::{gen_token, to_ptr, CodegenError, CodegenResult, Context, Object, Token, Type};
use llvm_sys::{core::*, *};

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
    unsafe {
        let result_object = context.scope.locals.entry(*var_name.clone()).or_insert({
            let result_value = LLVMBuildAlloca(
                context.builder,
                target.object_type.to_llvm_type(),
                to_ptr(&var_name),
            );
            Object::new(result_value, target.object_type)
        });
        LLVMBuildStore(context.builder, target.value, result_object.value);
        Ok(result_object.clone())
    }
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

pub fn while_production<'a, 'b>(
    context: &'a mut Context<'b>,
    args: &[Token],
) -> CodegenResult<Object> {
    if args.len() != 2 {
        return Err(CodegenError::new(&format!(
            "while expression should only have two arguments (conditional and body). found {}",
            args.len()
        )));
    };
    unsafe {
        // insert a basic block loop, to jump back to.
        let condition_block = LLVMAppendBasicBlockInContext(
            context.compiler.llvm_context,
            context.function,
            to_ptr("loop_condition"),
        );
        let loop_block = LLVMAppendBasicBlockInContext(
            context.compiler.llvm_context,
            context.function,
            to_ptr("loop"),
        );
        let after_loop_block = LLVMAppendBasicBlockInContext(
            context.compiler.llvm_context,
            context.function,
            to_ptr("after_loop"),
        );
        // go immediately to the block
        LLVMBuildBr(context.builder, condition_block);
        LLVMPositionBuilderAtEnd(context.builder, condition_block);
        let condition_result = gen_token(context, &args[0])?;
        LLVMBuildCondBr(
            context.builder,
            condition_result.value,
            loop_block,
            after_loop_block,
        );
        LLVMPositionBuilderAtEnd(context.builder, loop_block);
        let result = gen_token(context, &args[1])?;
        // loop back to conditional
        LLVMBuildBr(context.builder, condition_block);
        LLVMPositionBuilderAtEnd(context.builder, after_loop_block);
        Ok(result)
    }
}

pub fn add_production<'a, 'b>(
    context: &'a mut Context<'b>,
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
    let result = unsafe {
        LLVMBuildBinOp(
            context.builder,
            LLVMOpcode::LLVMAdd,
            lhs.value,
            rhs.value,
            to_ptr("addtemp"),
        )
    };
    Ok(Object::new(result, Type::Int))
}

pub fn not_production<'a, 'b>(
    context: &'a mut Context<'b>,
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
    let result =
        unsafe { LLVMBuildNot(context.builder, result_to_negate.value, to_ptr("nottemp")) };
    Ok(Object::new(result, Type::Bool))
}

pub fn match_production<'a, 'b>(
    context: &'a mut Context<'b>,
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
        let post_switch_block = LLVMAppendBasicBlockInContext(
            context.compiler.llvm_context,
            context.function,
            to_ptr("switchcomplete"),
        );
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
            let switch = LLVMBuildSwitch(
                context.builder,
                condition.value,
                post_switch_block,
                num_cases,
            );
            for (index, (_key, value)) in map.iter().enumerate() {
                let branch_block = LLVMAppendBasicBlockInContext(
                    context.compiler.llvm_context,
                    context.function,
                    to_ptr("case"),
                );
                let branch_key = &key_values[index];
                LLVMAddCase(switch, branch_key.value, branch_block);
                LLVMPositionBuilderAtEnd(context.builder, branch_block);
                // TODO: capture this value and make it the return value of the
                // match statement.
                gen_token(context, value)?;
                LLVMBuildBr(context.builder, post_switch_block);
            }
        } else {
            return Err(CodegenError::new(&format!(
                "match expression should be map. found {}",
                &args[1]
            )));
        }
        LLVMPositionBuilderAtEnd(context.builder, post_switch_block);
    }
    Ok(Object::none())
}

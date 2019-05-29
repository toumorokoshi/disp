use super::*;

pub fn expression() -> Expression {
    Expression {
        boostrap_compiler,
        typecheck,
        codegen,
    }
}

fn boostrap_compiler(_compiler: &mut Compiler) {}

fn typecheck(
    _resolver: &mut TypeResolver<TypecheckType>,
    _: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    // TODO: figure out how to recurse into nested
    // data structure type variables.
    Ok(args[0].clone())
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    if args.len() != 2 {
        return Err(CodegenError::new(&format!(
            "match expression should have two arguments: a value to match, and a map to match against. found {:?}",
            args
        )));
    };
    let condition = gen_token(context, &args[0])?;
    let post_switch_block = context.create_block("postswitch".to_owned());
    if let Token::Map(ref map) = &args[1] {
        let mut key_values = vec![];
        // we construct all keys first, to ensure
        // that they exist before the match statement is
        // executed.
        // TODO: reject keys that are not constants
        for key in map.keys() {
            let key_value = gen_token(context, &key.as_token())?;
            key_values.push(key_value);
        }
        let num_cases = (map.len() - 1) as u32;
        let switch = context.allocate_without_type();
        context.add_instruction(LLVMInstruction::BuildSwitch {
            value: condition.index,
            post_switch_block,
            num_cases,
            target: switch,
        });
        for (index, (_key, value)) in map.iter().enumerate() {
            let block = context.create_block("case".to_owned());
            let branch_key = &key_values[index];
            context.add_instruction(LLVMInstruction::AddCase {
                switch,
                value: branch_key.index,
                block: block,
            });
            let mut branch_context = Context::new(
                context.function_map,
                context.compiler,
                context.function,
                context.scope,
                block,
            );
            // context.add_instruction(LLVMInstruction::PositionBuilderAtEnd { block });
            // TODO: capture this value and make it the return value of the
            // match statement.
            gen_token(&mut branch_context, value)?;
            if !branch_context.current_block().has_been_terminated() {
                branch_context.add_instruction(LLVMInstruction::BuildBr {
                    block: post_switch_block,
                });
            }
        }
    } else {
        return Err(CodegenError::new(&format!(
            "match expression should be map. found {}",
            &args[1]
        )));
    }
    context.block = post_switch_block;
    Ok(Object::none())
}

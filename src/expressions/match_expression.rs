use super::*;

pub fn expression() -> Expression {
    Expression {
        boostrap_compiler,
        typecheck,
        codegen,
    }
}

fn boostrap_compiler(compiler: &mut Compiler) {}

fn typecheck(
    resolver: &mut TypeResolver<Type>,
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
    let post_switch_block = context.add_basic_block("switchcomplete".to_owned());
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
    Ok(Object::none())
}

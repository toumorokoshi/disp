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
    resolver.add_constraint(Constraint::Equality(args[0].clone(), args[1].clone()))?;
    Ok(args[0].clone())
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
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
    let result_object = context
        .scope
        .locals
        .entry(*var_name.clone())
        .or_insert({
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
        })
        .clone();
    context.add_instruction(LLVMInstruction::BuildStore {
        source: target.index,
        target: result_object.index,
    });
    Ok(result_object.clone())
}

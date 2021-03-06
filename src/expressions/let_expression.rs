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
    resolver: &mut TypeResolver<TypecheckType>,
    _: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    let type_var = resolver.create_type_var();
    // this is not invoked, instead there
    // is custom code for this one use case in the
    // type annotator.
    Ok(type_var)
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
            context.function.basic_blocks[context.block].add_instruction(
                LLVMInstruction::BuildAlloca {
                    llvm_type: context.compiler.llvm.types.get(&target.object_type),
                    target: object,
                },
            );
            Object::new(object, target.object_type.clone())
        })
        .clone();
    context.add_instruction(LLVMInstruction::BuildStore {
        source: target.index,
        target: result_object.index,
    });
    Ok(result_object.clone())
}

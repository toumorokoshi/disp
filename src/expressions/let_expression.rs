use super::*;
use inference::{Constraint, TypeResolver, TypeVar};

pub fn expression() -> Expression {
    Expression {
        typecheck: let_typecheck,
        codegen: let_codegen,
    }
}

fn let_typecheck(
    resolver: &mut TypeResolver<Type>,
    function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    resolver.add_constraint(Constraint::Equality(args[0].clone(), args[1].clone()))?;
    Ok(args[0].clone())
}

pub fn let_codegen(context: &mut Context, args: &[Token]) -> GenericResult<Object> {
    if args.len() != 2 {
        return Err(Box::new(CodegenError::new(&format!(
            "let function should only have two arguments. found {}: {:?}",
            args.len(),
            args
        ))));
    };
    let var_name = match &args[0] {
        Token::Symbol(ref s) => s.clone(),
        t => {
            return Err(Box::new(CodegenError::new(&format!(
                "expected a symbol for the first argument. found {}",
                t,
            ))));
        }
    };
    let target = gen_token(context, &args[1])?;
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

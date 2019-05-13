use super::*;

pub fn expression() -> Expression {
    Expression {
        boostrap_compiler: boostrap_compiler,
        typecheck: typecheck,
        codegen: codegen,
    }
}

fn boostrap_compiler(compiler: &mut Compiler) {
    add_function_to_compiler(
        compiler,
        "get",
        Type::Byte,
        &vec![Type::Array(Box::new(Type::Byte)), Type::Int],
        "get_bytes",
    );
}

fn typecheck(
    resolver: &mut TypeResolver<Type>,
    _function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    let type_var = resolver.create_type_var();
    resolver.add_constraint(Constraint::IsLiteral(type_var, Type::Byte))?;
    resolver.add_constraint(Constraint::IsLiteral(
        args[0],
        Type::Array(Box::new(Type::Byte)),
    ))?;
    resolver.add_constraint(Constraint::IsLiteral(args[1], Type::Int))?;
    Ok(type_var)
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    // let array = gen_token(context, &args[0])?;
    // let index = gen_token(context, &args[1])?;
    // // assign the array pointer first
    // let array_pointer = context.allocate_without_type();
    // context.add_instruction(LLVMInstruction::BuildGEP {
    //     value: array.index,
    //     // first element of object, first field (raw array)
    //     indices: vec![0, 0, ],
    //     target: array_pointer,
    // });
    // let result = context.allocate(Type::Byte);
    // context.add_instruction(LLVMInstruction::BuildStore {
    //     source: array_value_pointer,
    //     target: array_pointer,
    // });

    call_function(context, "get", args)
}

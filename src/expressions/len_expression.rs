use super::*;

pub fn expression() -> Expression {
    Expression {
        boostrap_compiler: boostrap_compiler,
        typecheck: typecheck,
        codegen: codegen,
    }
}

fn boostrap_compiler(_compiler: &mut Compiler) {}

fn typecheck(
    resolver: &mut TypeResolver<TypecheckType>,
    _function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    let type_var = resolver.create_type_var();
    let array_child = resolver.create_type_var();
    resolver.add_constraint(Constraint::IsLiteral(
        args[0],
        Unresolved::Generic(TypecheckType::Array, vec![array_child]),
    ))?;
    resolver.add_constraint(Constraint::IsLiteral(array_child, Unresolved::Any))?;
    resolver.add_constraint(Constraint::IsLiteral(
        type_var,
        Unresolved::Literal(TypecheckType::Int),
    ))?;
    Ok(type_var)
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    let array = gen_token(context, &args[0])?;
    let len_pointer_gep = context.allocate_without_type();
    let zero_value = context.const_i32(0).index;
    let one_value = context.const_i32(1).index;
    context.add_instruction(LLVMInstruction::BuildGEP {
        value: array.index,
        // first element of object, first field (raw array)
        indices: vec![zero_value, one_value],
        target: len_pointer_gep,
    });
    let length_value = context.allocate(Type::Int);
    context.add_instruction(LLVMInstruction::BuildLoad {
        source: len_pointer_gep,
        target: length_value.index,
    });
    return Ok(length_value);
}

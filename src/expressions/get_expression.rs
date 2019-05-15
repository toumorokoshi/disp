use super::*;
use llvm_sys::{core::*, execution_engine::*, prelude::*, support::*, target::*, *};

pub fn expression() -> Expression {
    Expression {
        boostrap_compiler: boostrap_compiler,
        typecheck: typecheck,
        codegen: codegen,
    }
}

fn boostrap_compiler(_compiler: &mut Compiler) {}

fn typecheck(
    resolver: &mut TypeResolver<Type>,
    _function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    // TODO: support ANY parameters for generics
    resolver.add_constraint(Constraint::IsLiteral(
        args[0],
        Type::Array(Box::new(Type::Any)),
    ))?;
    resolver.add_constraint(Constraint::IsLiteral(args[1], Type::Int))?;
    let type_var = resolver.create_type_var();
    // TODO: support Generic relationship constraints
    // resolver.add_constraint(Constraint::IsGenericOf(args[0], type_var));
    resolver.add_constraint(Constraint::IsLiteral(type_var, Type::Byte))?;
    Ok(type_var)
}

pub fn codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    let array = gen_token(context, &args[0])?;
    let index = gen_token(context, &args[1])?;
    // assign the array pointer first
    let array_pointer_gep = context.allocate_without_type();
    let zero_value = context.const_i32(0).index;
    context.add_instruction(LLVMInstruction::BuildGEP {
        value: array.index,
        // first element of object, first field (raw array)
        indices: vec![zero_value, zero_value],
        target: array_pointer_gep,
    });
    let array_pointer = context.allocate_without_type();
    let byte_pointer_type =
        unsafe { LLVMPointerType(LLVMInt8TypeInContext(context.compiler.llvm.context), 0) };
    context.add_instruction(LLVMInstruction::BuildAlloca {
        llvm_type: byte_pointer_type,
        target: array_pointer,
    });
    context.add_instruction(LLVMInstruction::BuildLoad {
        source: array_pointer_gep,
        target: array_pointer,
    });
    // next we use GEP again to pull the right index.
    let value_pointer = context.allocate_without_type();
    context.add_instruction(LLVMInstruction::BuildGEP {
        value: array_pointer,
        // first element of object, first field (raw array)
        indices: vec![index.index],
        target: value_pointer,
    });
    let result_type = match array.object_type {
        Type::Array(subtype) => (*subtype).clone(),
        other => {
            return Err(CodegenError::new(&format!(
                "type {:?} is not an array",
                other
            )));
        }
    };
    let llvm_byte_type = context.compiler.llvm.types.get(&result_type);
    let result = context.allocate(result_type);
    context.add_instruction(LLVMInstruction::BuildAlloca {
        llvm_type: llvm_byte_type,
        target: result.index,
    });
    context.add_instruction(LLVMInstruction::BuildLoad {
        source: value_pointer,
        target: result.index,
    });
    Ok(result)
}

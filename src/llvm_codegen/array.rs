/// Utility Code For Creating Disp Arrays (array + fat pointer)
/// in LLVM
use super::{extract_type_from_pointer, CodegenResult, Context, LLVMInstruction, Object, Type};

/// array_value_pointer should not be an actual pointer, but the
/// index in the scope in which the pointer actually lives.
pub fn create_array(
    context: &mut Context,
    subtype: &Type,
    array_value_pointer: usize,
    length: i64,
) -> CodegenResult<Object> {
    let array_type = Type::Array(Box::new(subtype.clone()));
    // the array type is a pointer to the struct, so
    // we need to get the actual struct type to construct
    // the underlying object.
    let struct_type = extract_type_from_pointer(context.compiler.llvm.types.get(&array_type));
    let object = context.allocate(array_type);
    context.add_instruction(LLVMInstruction::BuildAlloca {
        llvm_type: struct_type,
        target: object.index,
    });
    let zero_value = context.const_i32(0);
    let one_value = context.const_i32(1);
    // assign the array pointer first
    let array_pointer = context.allocate_without_type();
    context.add_instruction(LLVMInstruction::BuildGEP {
        value: object.index,
        // first element of object pointer, first field
        indices: vec![zero_value.index, zero_value.index],
        target: array_pointer,
    });
    context.add_instruction(LLVMInstruction::BuildStore {
        source: array_value_pointer,
        target: array_pointer,
    });
    // set the length next
    let length_pointer = context.allocate_without_type();
    let length_value = context.allocate(Type::Int);
    context.add_instruction(LLVMInstruction::BuildGEP {
        value: object.index,
        // first element of object pointer, second field
        indices: vec![zero_value.index, one_value.index],
        target: length_pointer,
    });
    context.add_instruction(LLVMInstruction::ConstInt {
        // value: s.len() as i64,
        value: length,
        target: length_value.index,
    });
    context.add_instruction(LLVMInstruction::BuildStore {
        source: length_value.index,
        target: length_pointer,
    });
    // finally, return the object
    Ok(object)
}

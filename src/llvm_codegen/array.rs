/// Utility Code For Creating Disp Arrays (array + fat pointer)
/// in LLVM
use super::{Type};


fn create_array(subtype: &Type) {
    let array_type = Type::Array(subtype.clone);
    // the array type is a pointer to the struct, so
    // we need to get the actual struct type to construct
    let struct_type =
        extract_type_from_pointer(context.compiler.llvm.types.get(&array_type));
    
}
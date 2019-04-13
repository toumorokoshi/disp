use llvm_sys::{core::*, prelude::*};
use super::{to_ptr};

// Declare the LLVM Array Object Type
pub fn llvm_declare_array(context: LLVMContextRef, base_type: LLVMTypeRef) -> LLVMTypeRef {
    // Our array struct is two values:
    // a pointer of the base type, representing the raw array
    // an i64 representing the type
    unsafe {
        let mut types = [base_type, LLVMInt64TypeInContext(context)];
        let struct_ref = LLVMStructCreateNamed(context, to_ptr("array"));
        LLVMStructSetBody(struct_ref, types.as_mut_ptr(), 2, 1);
        return struct_ref;
    }
}

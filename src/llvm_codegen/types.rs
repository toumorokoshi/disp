use llvm_sys::{
    core::*,
    prelude::*,
};

/// The type enum is used to define types for Disp's
/// type checker.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Int,
    None,
}

impl Type {
    // return the llvm equivalent of the
    // type.
    pub fn to_llvm_type(&self) -> LLVMTypeRef {
        unsafe {
            match self {
                &Type::Int => LLVMInt64Type(),
                &Type::None => LLVMVoidType(),
            }
        }
    }
}

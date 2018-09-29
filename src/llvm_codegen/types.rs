use llvm_sys::{core::*, prelude::*};

/// The type enum is used to define types for Disp's
/// type checker.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Int,
    None,
}

impl Into<LLVMTypeRef> for Type {
    fn into(self) -> LLVMTypeRef {
        self.to_llvm_type()
    }
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

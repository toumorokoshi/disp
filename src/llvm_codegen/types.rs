use llvm_sys::{core::*, prelude::*};

/// The type enum is used to define types for Disp's
/// type checker.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Bool,
    Int,
    None,
    String(usize),
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
                &Type::Bool => LLVMInt1Type(),
                &Type::Int => LLVMInt64Type(),
                &Type::None => LLVMVoidType(),
                &Type::String(count) => LLVMArrayType(LLVMInt8Type(), count as u32),
            }
        }
    }
}

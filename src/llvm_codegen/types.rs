use llvm_sys::{core::*, prelude::*};

/// The type enum is used to define types for Disp's
/// type checker.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Bool,
    Byte,
    Bytes,
    FunctionPrototype,
    Int,
    None,
    String,
    Map(Box<Type>, Box<Type>),
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
                &Type::Bytes => LLVMPointerType(LLVMInt8Type(), 0),
                &Type::Byte => LLVMInt8Type(),
                &Type::FunctionPrototype => LLVMVoidType(),
                &Type::Int => LLVMInt64Type(),
                &Type::None => LLVMVoidType(),
                &Type::String => LLVMPointerType(LLVMInt8Type(), 0),
                &Type::Map(ref k, ref v) => LLVMPointerType(LLVMVoidType(), 0),
            }
        }
    }
}

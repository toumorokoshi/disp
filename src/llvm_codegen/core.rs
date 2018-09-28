use llvm_sys::{
    core::*,
    prelude::*
};
use super::{
    Scope,
    LLVMBuilder,
    Type
};

/// Objects are to represent values,
/// variables, and functions.
pub struct Object {
    pub value: LLVMValueRef,
    pub object_type: Type,
}

impl Object {
    pub fn new(value: LLVMValueRef, object_type: Type) -> Object {
        Object{value: value, object_type: object_type}
    }

    pub fn none() -> Object {
        unsafe {
            Object::new(LLVMConstNull(LLVMVoidType()), Type::None)
        }
    }
}

/// Functions represent functions within disp.
#[derive(Clone)]
pub struct Function {
    pub arg_types: Vec<Type>,
    pub return_type: Type,
    // the LLVM function handle.
    pub function: LLVMValueRef,
}

/// The context object contains all relevant
/// information for the Codegen to successfully build
/// llvm code.
pub struct Context {
    pub compiler: Compiler,
    pub scope: Scope,
    pub builder: LLVMBuilder
}

impl Context {
    pub fn new(compiler: Compiler) -> Context {
        Context {
            compiler: compiler,
            scope: Scope::new(None),
            builder: LLVMBuilder::new(),
        }
    }
}

// the dispcompiler object is a global
/// that contains context for the whole
/// disp application being created.
pub struct Compiler {
    pub llvm_context: LLVMContextRef
}

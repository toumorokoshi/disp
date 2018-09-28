use llvm_sys::{
    prelude::*
};

pub struct Object {
    value: LLVMValueRef,
}

impl Object {
    pub fn new(value: LLVMValueRef) -> Object {
        Object{value: value}
    }
}

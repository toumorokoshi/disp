/// functions to be used in the llvm-compiled version of disp.
use llvm_sys::{
    core::*,
    prelude::*,
};
use super::{to_ptr};

/// Add external declarations to a module.
pub fn add_externs(module: LLVMModuleRef) {
    unsafe {
        let mut args = vec![LLVMInt64Type()];
        LLVMAddFunction(module, to_ptr("print"), LLVMFunctionType(
            LLVMInt64Type(), args.as_mut_ptr(), args.len() as u32, 0
        ));
    }
}

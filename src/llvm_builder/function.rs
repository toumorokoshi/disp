/// functions to be used in the llvm-compiled version of disp.
use llvm_sys::{
    core::*,
    prelude::*,
};
use super::{to_ptr};

extern "C" fn putsyusuke() {
    println!("foo");
    println!("bar");
}

/// Add external declarations to a module.
pub fn add_externs(module: LLVMModuleRef) {
    unsafe {
        let mut args = vec![];
        LLVMAddFunction(module, to_ptr("putsyusuke"), LLVMFunctionType(
            LLVMVoidType(), args.as_mut_ptr(), 0, 0
        ));
    }
}

use llvm_sys::{core::*, execution_engine::*, prelude::*, support::*, target::*, *};
use super::{to_ptr, Type, LLVMTypeCache};
use std::ptr;

pub struct LLVMCompiler {
    pub context: LLVMContextRef,
    pub module: LLVMModuleRef,
    pub builder: LLVMBuilderRef,
    pub types: LLVMTypeCache,
}

impl LLVMCompiler {
    pub fn new() -> LLVMCompiler {
        unsafe {
            // It would be nice to create a new context here. However,
            // earlier in the code types are already created. These types
            // are built within the global context, which is the context passed
            // into functions. As such, creating a new context would create a
            // context mismatch between the function (global) context and the
            // context used by the rest of the builder.
            let context = LLVMGetGlobalContext();
            // This is required to ensure that exported
            // functions are available to the context.
            LLVMLoadLibraryPermanently(ptr::null());
            let module = LLVMModuleCreateWithNameInContext(to_ptr("main"), context);
            let builder = LLVMCreateBuilderInContext(context);
            // TODO: figure out the right organization
            // for LLVM objects and codegen objects... strongly
            // itertwined.
            let types = LLVMTypeCache::new(context);
            return LLVMCompiler {
                context,
                module,
                builder,
                types
            };
        }
    }
}
mod utils;
mod function;
use llvm_sys::{
    self,
    core::{self, *},
    execution_engine,
    support::*,
    target,
    prelude::*,
};
use std::{
    collections::HashMap,
    mem,
    ptr,
};
pub use self::utils::to_ptr;
use self::function::{add_externs};

pub struct LLVMBuilder {
    pub context: LLVMContextRef,
    pub module: LLVMModuleRef,
    pub builder: LLVMBuilderRef,
}

impl LLVMBuilder {
    pub fn new() -> LLVMBuilder {
        unsafe {
            let context = core::LLVMContextCreate();
            let module = core::LLVMModuleCreateWithNameInContext(to_ptr("main"), context);
            add_externs(module);
            let builder = core::LLVMCreateBuilderInContext(context);
            // call this to ensure that the current binary is loaded as a
            // shared library, exposing all of it's externs.
            LLVMLoadLibraryPermanently(ptr::null());
            return LLVMBuilder {
                context: context,
                module: module,
                builder: builder,
            };
        }
    }

    /// build the body of the main function, for now.
    pub fn build_function(&self) {
        unsafe {
            let mut args = vec![];
            let function_type = core::LLVMFunctionType(
                core::LLVMVoidType(),
                args.as_mut_ptr(),
                args.len() as u32,
                0
            );
            let function = core::LLVMAddFunction(self.module, to_ptr("main"), function_type);
            let basic_block = core::LLVMAppendBasicBlockInContext(self.context,  function, to_ptr("entry"));
            // call puts
            let prints_function = LLVMGetNamedFunction(self.module, to_ptr("print"));
            let mut args = vec![
                LLVMConstInt(LLVMInt64Type(), 10 as u64, 0)
            ];
            core::LLVMPositionBuilderAtEnd(self.builder, basic_block);
            LLVMBuildCall(self.builder, prints_function, args.as_mut_ptr(), args.len() as u32, to_ptr(""));
            LLVMBuildRetVoid(self.builder);
        }
    }

    /// run the module. In the future, this will
    /// be replaced by simply compiling and writing out an executable.
    /// Although, this may remain as we need to run code during compile time.
    pub fn run(&self) {
        unsafe {
            core::LLVMDumpModule(self.module);
            let mut ee = mem::uninitialized();
            let mut out = mem::zeroed();
            execution_engine::LLVMLinkInMCJIT();
            target::LLVM_InitializeNativeTarget();
            target::LLVM_InitializeNativeAsmPrinter();
            execution_engine::LLVMCreateExecutionEngineForModule(&mut ee, self.module, &mut out);
            let addr = execution_engine::LLVMGetFunctionAddress(ee, to_ptr("main"));
            let f: extern "C" fn() = mem::transmute(addr);
            f();
            execution_engine::LLVMDisposeExecutionEngine(ee);
        }
    }

    pub fn cleanup(&self) {
        unsafe {
            llvm_sys::core::LLVMDisposeBuilder(self.builder);
            llvm_sys::core::LLVMDisposeModule(self.module);
            llvm_sys::core::LLVMContextDispose(self.context);
        }
    }
}

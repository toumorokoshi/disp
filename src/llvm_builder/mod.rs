use llvm_sys;
use std::ptr;

pub fn build_function() {
    unsafe {
        // Set up a context, module and builder in that context.
        let context = llvm_sys::core::LLVMContextCreate();
        let module = llvm_sys::core::LLVMModuleCreateWithName(b"nop\0".as_ptr() as *const _);
        let builder = llvm_sys::core::LLVMCreateBuilderInContext(context);

        // Get the type signature for void nop(void);
        // Then create it in our module.
        let void = llvm_sys::core::LLVMVoidTypeInContext(context);
        let function_type = llvm_sys::core::LLVMFunctionType(void, ptr::null_mut(), 0, 0);
        let function = llvm_sys::core::LLVMAddFunction(module, b"nop\0".as_ptr() as *const _,
                                                   function_type);

        // Create a basic block in the function and set our builder to generate
        // code in it.
        let bb = llvm_sys::core::LLVMAppendBasicBlockInContext(context, function,
                                                           b"entry\0".as_ptr() as *const _);
        llvm_sys::core::LLVMPositionBuilderAtEnd(builder, bb);

        // Emit a `ret void` into the function
        llvm_sys::core::LLVMBuildRetVoid(builder);

        // Dump the module as IR to stdout.
        llvm_sys::core::LLVMDumpModule(module);

        // Clean up. Values created in the context mostly get cleaned up there.
        llvm_sys::core::LLVMDisposeBuilder(builder);
        llvm_sys::core::LLVMDisposeModule(module);
        llvm_sys::core::LLVMContextDispose(context);
    };
}

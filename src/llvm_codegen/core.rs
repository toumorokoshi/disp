use super::{add_native_functions, to_ptr, FunctionPrototype, Scope, Type};
use llvm_sys::{core::*, prelude::*, support::*};
use std::ptr;

/// Objects are to represent values,
/// variables, and functions.
#[derive(Clone, Debug)]
pub struct Object {
    pub value: LLVMValueRef,
    pub object_type: Type,
    pub function_prototype: Option<FunctionPrototype>,
}

impl Object {
    pub fn new(value: LLVMValueRef, object_type: Type) -> Object {
        Object {
            value: value,
            object_type: object_type,
            function_prototype: None,
        }
    }

    pub fn function_prototype(function_prototype: FunctionPrototype) -> Object {
        Object {
            value: ptr::null_mut(),
            object_type: Type::FunctionPrototype,
            function_prototype: Some(function_prototype),
        }
    }

    pub fn none() -> Object {
        unsafe { Object::new(LLVMConstNull(LLVMVoidType()), Type::None) }
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
/// llvm ode.
pub struct Context<'a, 'b: 'a> {
    pub scope: &'a mut Scope<'b>,
    pub builder: LLVMBuilderRef,
    pub function: LLVMValueRef,
    /// this should be the current block that
    /// the builder is building against. This allows
    /// one to get back to it when switching context,
    /// for example building a child function.
    pub block: LLVMBasicBlockRef,
    pub module: LLVMModuleRef,
    pub llvm_context: LLVMContextRef,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn new(
        llvm_context: LLVMContextRef,
        module: LLVMModuleRef,
        scope: &'a mut Scope<'b>,
        builder: LLVMBuilderRef,
        function: LLVMValueRef,
        block: LLVMBasicBlockRef,
    ) -> Context<'a, 'b> {
        Context {
            llvm_context: llvm_context,
            module: module,
            scope: scope,
            builder: builder,
            function: function,
            block: block,
        }
    }
}

// the dispcompiler object is a global
/// that contains context for the whole
/// disp application being created.
pub struct Compiler<'a> {
    pub llvm_context: LLVMContextRef,
    pub llvm_module: LLVMModuleRef,
    pub llvm_builder: LLVMBuilderRef,
    pub scope: Scope<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Compiler<'a> {
        unsafe {
            let context = LLVMContextCreate();
            // This is required to ensure that exported
            // functions area available to the context.
            LLVMLoadLibraryPermanently(ptr::null());
            let module = LLVMModuleCreateWithNameInContext(to_ptr("main"), context);
            let builder = LLVMCreateBuilderInContext(context);
            let mut compiler = Compiler {
                llvm_context: context,
                llvm_module: module,
                llvm_builder: builder,
                scope: Scope::new(None),
            };
            // add_native_functions(&mut compiler);
            compiler
        }
    }
}

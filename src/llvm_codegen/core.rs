use super::{Scope, Type};
use llvm_sys::{core::*, prelude::*, support::*};
use std::ptr;

/// Objects are to represent values,
/// variables, and functions.
#[derive(Clone, Debug)]
pub struct Object {
    pub value: LLVMValueRef,
    pub object_type: Type,
}

impl Object {
    pub fn new(value: LLVMValueRef, object_type: Type) -> Object {
        Object {
            value: value,
            object_type: object_type,
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
/// llvm code.
pub struct Context<'a> {
    pub compiler: &'a mut Compiler,
    pub scope: &'a mut Scope<'a>,
    pub module: LLVMModuleRef,
    pub builder: LLVMBuilderRef,
    pub function: LLVMValueRef,
}

impl<'a> Context<'a> {
    pub fn new(
        compiler: &'a mut Compiler,
        scope: &'a mut Scope<'a>,
        module: LLVMModuleRef,
        builder: LLVMBuilderRef,
        function: LLVMValueRef,
    ) -> Context<'a> {
        Context {
            compiler: compiler,
            scope: scope,
            builder: builder,
            module: module,
            function: function,
        }
    }
}

// the dispcompiler object is a global
/// that contains context for the whole
/// disp application being created.
pub struct Compiler {
    pub llvm_context: LLVMContextRef,
}

impl Compiler {
    pub fn new() -> Compiler {
        unsafe {
            let context = LLVMContextCreate();
            // This is required to ensure that exported
            // functions area available to the context.
            LLVMLoadLibraryPermanently(ptr::null());
            Compiler {
                llvm_context: context,
            }
        }
    }
}

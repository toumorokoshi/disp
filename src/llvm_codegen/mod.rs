mod array;
pub mod compiler;
mod context;
mod functions;
mod core;
mod error;
mod llvm_context;
mod scope;
mod types;
mod utils;
pub use self::array::create_array;
pub use self::compiler::build_functions;
pub use self::context::Context;
pub use self::core::{BasicBlock, Function, FunctionType, NativeFunction, Object};
pub use self::error::{CodegenError, CodegenResult};
pub use self::llvm_context::LLVMCompiler;
pub use self::scope::Scope;
pub use self::types::LLVMTypeCache;
pub use self::utils::*;
use super::{
    get_builtin_expressions, AnnotatedFunction, AnnotatedFunctionMap, CompilerData,
    LLVMInstruction, Token, Type,
};
use llvm_sys::{analysis::*, core::*, execution_engine::*, prelude::*, support::*, target::*, *};
use std::collections::HashSet;

/// Construct the LLVM module and native functions
pub fn build(compiler: &mut Compiler, functions: &AnnotatedFunctionMap) {
    // the first step is iterating through all codegen functions,
    // building their function objects.
    // this ensures that invocations are referenced.
    self::functions::populate_function_pointers(&mut compiler.llvm, functions);
    // now we sequentially construct each function's IR
}

// the dispcompiler object is a global
/// that contains context for the whole
/// disp application being created.
pub struct Compiler<'a> {
    pub llvm: LLVMCompiler,
    pub scope: Scope<'a>,
    pub data: CompilerData,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Compiler<'a> {
        let mut compiler = Compiler {
            llvm: LLVMCompiler::new(),
            scope: Scope::new(None),
            data: CompilerData::new(),
        };
        for expression in get_builtin_expressions().values() {
            (expression.boostrap_compiler)(&mut compiler);
        }
        compiler
    }
}

pub fn build_native_function(
    function: &NativeFunction,
    module: *mut LLVMModule,
    types: &mut LLVMTypeCache,
) {
    let mut llvm_args = Vec::with_capacity(function.arg_types.len());
    for arg in &function.arg_types {
        llvm_args.push(types.get(&arg));
    }
    unsafe {
        let llvm_function = LLVMGetNamedFunction(module, to_ptr(&function.name));
        if llvm_function.is_null() {
            LLVMAddFunction(
                module,
                to_ptr(&function.name),
                LLVMFunctionType(
                    types.get(&function.return_type),
                    llvm_args.as_mut_ptr(),
                    llvm_args.len() as u32,
                    0,
                ),
            );
        }
    }
}

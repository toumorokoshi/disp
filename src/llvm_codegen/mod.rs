pub mod compiler;
mod context;
mod core;
mod error;
mod llvm_context;
mod scope;
mod types;
mod utils;
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

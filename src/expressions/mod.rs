use self::utils::codegen_binop;
use super::llvm_codegen::{compiler::gen_token, CodegenError, CodegenResult, Compiler, Scope};
use super::{
    Array, CompilerData, Context, FunctionType, GenericResult, LLVMInstruction, NativeFunction,
    Object, Token, Type, TypecheckType, TypevarFunction, AnnotatorScope
};
use inference::{Constraint, TypeResolver, TypeVar, Unresolved};
use libc::c_char;
use llvm_sys::*;
use std::{collections::HashMap, ffi::CStr};

mod add_expression;
mod bytes_builtins;
pub use self::bytes_builtins::*;
mod eq_expression;
mod get_expression;
mod int_expression;
mod len_expression;
mod let_expression;
mod match_expression;
mod not_expression;
mod print_expression;
mod readline_expression;
mod while_expression;
pub use self::int_expression::*;
pub use self::not_expression::*;
pub use self::print_expression::*;
pub use self::readline_expression::*;
mod return_expression;
mod subtract_expression;
mod utils;
use self::utils::*;
/// This module contains all the expressions that are
/// built in.
/// Builtin expressions require a couple components:
/// 1. A TypeCheck Function to help type resolution
/// 2. A Codegen Function
pub type BuiltinExpressions = HashMap<String, Expression>;
pub struct Expression {
    pub boostrap_compiler: fn(&mut Compiler),
    pub typecheck: fn(
        resolver: &mut TypeResolver<TypecheckType>,
        function: &TypevarFunction,
        args: &Vec<TypeVar>,
    ) -> GenericResult<TypeVar>,
    pub codegen: fn(&mut Context, &[Token]) -> CodegenResult<Object>,
}
/// Return all expressions
pub fn get_builtin_expressions() -> BuiltinExpressions {
    let mut expressions = HashMap::new();
    expressions.insert(String::from("-"), subtract_expression::expression());
    expressions.insert(String::from("+"), add_expression::expression());
    expressions.insert(String::from("eq"), eq_expression::expression());
    expressions.insert(String::from("get"), get_expression::expression());
    expressions.insert(String::from("int"), int_expression::expression());
    expressions.insert(String::from("let"), let_expression::expression());
    expressions.insert(String::from("len"), len_expression::expression());
    expressions.insert(String::from("match"), match_expression::expression());
    expressions.insert(String::from("not"), not_expression::expression());
    expressions.insert(String::from("print"), print_expression::expression());
    expressions.insert(String::from("return"), return_expression::expression());
    expressions.insert(String::from("read-line"), readline_expression::expression());
    expressions.insert(String::from("while"), while_expression::expression());
    expressions
}

use super::llvm_codegen::{
    compiler::{gen_token, Context},
    CodegenError, CodegenResult,
};
use super::{GenericResult, LLVMInstruction, Object, Token, Type, TypevarFunction};
use inference::{Constraint, TypeResolver, TypeVar};
use std::collections::HashMap;

mod let_expression;
mod return_expression;
/// This module contains all the expressions that are
/// built in.
/// Builtin expressions require a couple components:
/// 1. A TypeCheck Function to help type resolution
/// 2. A Codegen Function
pub type BuiltinExpressions = HashMap<String, Expression>;
pub struct Expression {
    pub typecheck:
        fn(resolver: &mut TypeResolver<Type>, function: &TypevarFunction, args: &Vec<TypeVar>)
            -> GenericResult<TypeVar>,
    pub codegen: fn(&mut Context, &[Token]) -> CodegenResult<Object>,
}
/// Return all expressions
pub fn get_builtin_expressions() -> BuiltinExpressions {
    let mut expressions = HashMap::new();
    expressions.insert(String::from("let"), let_expression::expression());
    expressions.insert(String::from("return"), return_expression::expression());
    expressions.insert(
        String::from("match"),
        Expression {
            typecheck: match_typecheck,
            codegen: empty_codegen,
        },
    );
    expressions.insert(
        String::from("-"),
        Expression {
            typecheck: subtract_typecheck,
            codegen: empty_codegen,
        },
    );
    expressions.insert(
        String::from("+"),
        Expression {
            typecheck: add_typecheck,
            codegen: empty_codegen,
        },
    );
    expressions
}

pub fn match_typecheck(
    resolver: &mut TypeResolver<Type>,
    function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    // TODO: figure out how to recurse into nested
    // data structure type variables.
    Ok(args[0].clone())
}

pub fn subtract_typecheck(
    resolver: &mut TypeResolver<Type>,
    function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    // TODO: figure out how to recurse into nested
    // data structure type variables.
    resolver.add_constraint(Constraint::Equality(args[1].clone(), args[0].clone()))?;
    Ok(args[0].clone())
}

pub fn add_typecheck(
    resolver: &mut TypeResolver<Type>,
    function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    // TODO: figure out how to recurse into nested
    // data structure type variables.
    resolver.add_constraint(Constraint::Equality(args[1].clone(), args[0].clone()))?;
    Ok(args[0].clone())
}

pub fn empty_codegen(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    return Err(CodegenError::new(&format!("unimplemented type",)));
}

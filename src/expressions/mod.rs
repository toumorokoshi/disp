use super::{DispResult, Type};
use inference::{Constraint, TypeResolver, TypeVar};
use std::collections::HashMap;
/// This module contains all the expressions that are
/// built in.
/// Builtin expressions require a couple components:
/// 1. A TypeCheck Function to help type resolution
/// 2. A Codegen Function
pub type BuiltinExpressions = HashMap<String, Expression>;
pub struct Expression {
    pub typecheck:
        fn(resolver: &mut TypeResolver<Type>, args: &Vec<TypeVar>) -> DispResult<TypeVar>,
}
/// Return all expressions
pub fn get_builtin_expressions() -> BuiltinExpressions {
    let mut expressions = HashMap::new();
    expressions.insert(
        String::from("let"),
        Expression {
            typecheck: let_typecheck,
        },
    );
    expressions
}

pub fn let_typecheck(
    resolver: &mut TypeResolver<Type>,
    args: &Vec<TypeVar>,
) -> DispResult<TypeVar> {
    resolver.add_constraint(Constraint::Equality(args[0].clone(), args[1].clone()));
    Ok(args[0].clone())
}

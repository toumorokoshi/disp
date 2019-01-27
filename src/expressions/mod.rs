use super::{GenericResult, Type, TypevarFunction};
use inference::{Constraint, TypeResolver, TypeVar};
use std::collections::HashMap;
/// This module contains all the expressions that are
/// built in.
/// Builtin expressions require a couple components:
/// 1. A TypeCheck Function to help type resolution
/// 2. A Codegen Function
pub type BuiltinExpressions = HashMap<String, Expression>;
pub struct Expression {
    pub typecheck: fn(
        resolver: &mut TypeResolver<Type>,
        function: &TypevarFunction,
        args: &Vec<TypeVar>,
    ) -> GenericResult<TypeVar>,
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
    expressions.insert(
        String::from("return"),
        Expression {
            typecheck: return_typecheck,
        },
    );
    expressions.insert(
        String::from("match"),
        Expression {
            typecheck: match_typecheck,
        },
    );
    expressions.insert(
        String::from("-"),
        Expression {
            typecheck: subtract_typecheck,
        },
    );
    expressions.insert(
        String::from("+"),
        Expression {
            typecheck: add_typecheck,
        },
    );
    expressions
}

pub fn let_typecheck(
    resolver: &mut TypeResolver<Type>,
    function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    resolver.add_constraint(Constraint::Equality(args[0].clone(), args[1].clone()))?;
    Ok(args[0].clone())
}

pub fn return_typecheck(
    resolver: &mut TypeResolver<Type>,
    function: &TypevarFunction,
    args: &Vec<TypeVar>,
) -> GenericResult<TypeVar> {
    resolver.add_constraint(Constraint::Equality(
        function.return_type.clone(),
        args[0].clone(),
    ))?;
    Ok(args[0].clone())
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

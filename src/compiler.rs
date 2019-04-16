use super::{get_builtin_expressions, BuiltinExpressions, FunctionType};
use std::collections::HashMap;

pub struct CompilerData {
    pub functions: HashMap<String, FunctionType>,
    pub builtin_expressions: BuiltinExpressions,
}

impl CompilerData {
    pub fn new() -> CompilerData {
        CompilerData {
            functions: HashMap::new(),
            builtin_expressions: get_builtin_expressions(),
        }
    }
}

/// The type enum is used to define types for Disp's
/// type checker.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Bool,
    Byte,
    Bytes,
    FunctionPrototype,
    Int,
    None,
    String,
    Map(Box<Type>, Box<Type>),
}

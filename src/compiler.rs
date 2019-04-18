use super::{get_builtin_expressions, BuiltinExpressions, FunctionType, TypeSet};
use std::collections::{HashMap};

pub struct CompilerData {
    pub functions: HashMap<String, FunctionType>,
    pub builtin_expressions: BuiltinExpressions,
    pub types: TypeSet,
}

impl CompilerData {
    pub fn new() -> CompilerData {
        CompilerData {
            functions: HashMap::new(),
            builtin_expressions: get_builtin_expressions(),
            types: TypeSet::new(),
        }
    }
}
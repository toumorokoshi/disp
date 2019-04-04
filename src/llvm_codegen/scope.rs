use super::{Object, Type};
use std::collections::HashMap;

/// Scope objects handle references to functions by value.
pub struct Scope<'a> {
    pub locals: HashMap<String, Object>,
    /// a declaration of functions, including
    /// polymorphism for type definitions.
    pub functions: HashMap<String, HashMap<Vec<Type>, String>>,
    // this can reference parent scopes.
    pub parent: Option<&'a Scope<'a>>,
}

impl<'a> Scope<'a> {
    pub fn new(parent: Option<&'a Scope<'a>>) -> Scope<'a> {
        Scope {
            locals: HashMap::new(),
            functions: HashMap::new(),
            parent: parent,
        }
    }

    pub fn add_function(&mut self, name: &str, arg_types: &[Type], llvm_name: String) {
        {
            if let Some(ref mut map) = self.functions.get_mut(name) {
                map.insert(arg_types.to_owned(), llvm_name);
                return;
            }
        }
        let mut map = HashMap::new();
        map.insert(arg_types.to_owned(), llvm_name);
        self.functions.insert(name.to_string(), map);
    }

    pub fn get_function(&self, name: &str, arg_types: &[Type]) -> Option<String> {
        let maybe_function = match self.functions.get(name) {
            Some(functions_by_type_signature) => match functions_by_type_signature.get(arg_types) {
                Some(func) => Some(func.clone()),
                None => None,
            },
            None => None,
        };
        match maybe_function {
            Some(function) => Some(function),
            None => match self.parent {
                Some(scope) => scope.get_function(name, arg_types),
                None => None,
            },
        }
    }

    pub fn get_local(&self, key: &str) -> Option<Object> {
        match self.locals.get(key) {
            Some(o) => Some(o.clone()),
            None => None,
        }
    }
}

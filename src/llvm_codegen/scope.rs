use super::{Function, FunctionPrototype, Object, Token, Type};
use llvm_sys::prelude::*;
use std::collections::HashMap;

/// Scope objects handle references to functions by value.
pub struct Scope<'a> {
    pub locals: HashMap<String, Object>,
    /// a declaration of functions, including
    /// polymorphism for type definitions.
    pub functions: HashMap<String, HashMap<Vec<Type>, Function>>,
    pub function_prototypes: HashMap<String, FunctionPrototype>,
    // this can reference parent scopes.
    pub parent: Option<&'a Scope<'a>>,
}

impl<'a> Scope<'a> {
    pub fn new(parent: Option<&'a Scope<'a>>) -> Scope<'a> {
        Scope {
            locals: HashMap::new(),
            functions: HashMap::new(),
            function_prototypes: HashMap::new(),
            parent: parent,
        }
    }

    pub fn add_function(&mut self, name: &str, function: Function) {
        {
            if let Some(ref mut map) = self.functions.get_mut(name) {
                map.insert(function.arg_types.clone(), function);
                return;
            }
        }
        let mut map = HashMap::new();
        map.insert(function.arg_types.clone(), function);
        self.functions.insert(name.to_string(), map);
    }

    pub fn get_function(&self, name: &str, arg_types: &Vec<Type>) -> Option<Function> {
        match self.functions.get(name) {
            Some(functions_by_type_signature) => match functions_by_type_signature.get(arg_types) {
                Some(func) => Some(func.clone()),
                None => None,
            },
            None => None,
        }
    }

    pub fn get_prototype(&self, name: &str) -> Option<FunctionPrototype> {
        match self.function_prototypes.get(name) {
            Some(prototype) => Some(prototype.clone()),
            None => None,
        }
    }
}

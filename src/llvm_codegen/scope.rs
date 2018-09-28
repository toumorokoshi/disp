use std::collections::HashMap;
use llvm_sys::prelude::*;
use super::{
    Function,
    FunctionPrototype,
    Token,
    Type
};

/// Scope objects handle references to functions by value.
pub struct Scope {
    pub locals: HashMap<String, LLVMValueRef>,
    /// a declaration of functions, including
    /// polymorphism for type definitions.
    pub functions: HashMap<String, HashMap<Vec<Type>, Function>>,
    pub function_prototypes: HashMap<String, FunctionPrototype>,
    // this can reference parent scopes.
    pub parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn new(parent: Option<Box<Scope>>) -> Scope {
        Scope {
            locals: HashMap::new(),
            functions: HashMap::new(),
            function_prototypes: HashMap::new(),
            parent: parent,
        }
    }

    pub fn add_function(&self, name: &str, function: Function) {
        if let Some(ref mut map) = self.functions.get_mut(name) {
            map.insert(function.arg_types.clone(), function);
        } else {
            let map = HashMap::new();
            map.insert(function.arg_types.clone(), function);
            self.functions.insert(name.to_string(), map);
        }
    }

    pub fn get_function(&self, name: &str, arg_types: &Vec<Type>) -> Option<Function> {
        match self.functions.get(name) {
            Some(functions_by_type_signature) => {
                match functions_by_type_signature.get(arg_types) {
                    Some(func) => Some(func.clone()),
                    None => None,
                }
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

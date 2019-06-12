use inference::TypeVar;
use std::collections::HashMap;

pub struct AnnotatorScope<'a> {
    pub parent: Option<&'a AnnotatorScope<'a>>,
    pub locals: HashMap<String, TypeVar>,
}

impl<'a> AnnotatorScope<'a> {
    pub fn new() -> AnnotatorScope<'a> {
        AnnotatorScope {
            parent: None,
            locals: HashMap::new(),
        }
    }

    /// get a variable.
    pub fn get(&self, name: &String) -> Option<TypeVar> {
        match self.locals.get(name) {
            Some(type_var) => Some(type_var.clone()),
            None => match self.parent {
                Some(ref parent_scope) => parent_scope.get(name),
                None => None,
            },
        }
    }
}

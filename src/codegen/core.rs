use super::super::Token;
use super::{CodegenError, FunctionPrototype};
/// thoughts here:
/// - Object probably needs be disp-specific to allow
///   disp-specific functionality, or the ghvm builder
///   needs to provide more rules about type construction.
use std::collections::HashMap;
use warpspeed::{BuildObject, FunctionBuilder, Type, VM};

pub type Production = fn(context: &mut Context, args: &[Token]) -> CodegenResult;
pub type CodegenResult = Result<Object, CodegenError>;
pub fn function_prototype() -> Type {
    Type::Function(Box::new(Vec::new()), Box::new(Type::None))
}

/// function prototypes are an unevaluated declaration
/// of a function. These are used to generate actual
/// functions within the VM, when types are passed.
pub struct Block {
    // string w / register
    pub locals: HashMap<String, usize>,
    pub function_prototypes: Vec<FunctionPrototype>,
}

impl Block {
    pub fn new() -> Block {
        let block = Block {
            locals: HashMap::new(),
            function_prototypes: vec![],
        };
        return block;
    }

    pub fn get_local(&self, key: &String) -> Option<usize> {
        match self.locals.get(key) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}

pub struct Context<'a> {
    pub block: Block,
    pub builder: FunctionBuilder,
    pub vm: &'a mut VM,
}

impl<'a> Context<'a> {
    pub fn new(vm: &'a mut VM) -> Context {
        return Context {
            block: Block::new(),
            builder: FunctionBuilder::new(),
            vm: vm,
        };
    }
}

#[derive(Clone, Debug)]
pub struct Object {
    pub typ: Type, // the type of the register
    // the register containing the value.
    pub register: usize,
    pub function_index: Option<usize>,
}

impl Object {
    pub fn new(typ: Type, register: usize) -> Object {
        Object {
            typ: typ,
            register: register,
            function_index: None,
        }
    }

    pub fn none() -> Object {
        Object::new(Type::None, 0)
    }

    pub fn from_build_object(build_object: BuildObject) -> Object {
        return Object {
            typ: build_object.typ,
            register: build_object.register,
            function_index: None,
        };
    }

    pub fn to_build_object(&self) -> BuildObject {
        return BuildObject {
            typ: self.typ.clone(),
            register: self.register,
        };
    }
}

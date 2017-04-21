/// thoughts here:
/// - Object probably needs be disp-specific to allow
///   disp-specific functionality, or the ghvm builder
///   needs to provide more rules about type construction.

use ghvm;
use super::super::Block;

pub struct Context {
    pub block: Block,
    pub builder: ghvm::FunctionBuilder,
    pub vm: ghvm::VM
}

impl Context {
    pub fn new() -> Context {
        return Context {
            block: Block::new(),
            builder: ghvm::FunctionBuilder::new(),
            vm: ghvm::VM::new()
        }
    }
}

pub struct Object {
    typ: ghvm::Type, // the type of the register
    register: usize // the register containing the value
}

impl Object {
    pub fn from_build_object(build_object: ghvm::BuildObject) -> Object {
        return Object {
            typ: build_object.typ,
            register: build_object.register
        };
    }

    pub fn to_build_object(&self) -> ghvm::BuildObject {
        return ghvm::BuildObject {
            typ: self.typ.clone(),
            register: self.register
        };
    }
}

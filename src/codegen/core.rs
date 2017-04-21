/// thoughts here:
/// - Object probably needs be disp-specific to allow
///   disp-specific functionality, or the ghvm builder
///   needs to provide more rules about type construction.

use ghvm;
use super::super::Block;
use super::types::Type;

pub struct Context {
    block: Block,
    builder: ghvm::FunctionBuilder,
    vm: ghvm::VM
}

impl Context {
    pub fn new() -> Context {
        return Context {
            block: Block::new(),
            function: ghvm::Function::new(),
            vm:: ghvm::VM::new()
        }
    }
}

struct Object {
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
}

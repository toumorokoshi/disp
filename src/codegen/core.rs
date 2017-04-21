/// thoughts here:
/// - Object probably needs be disp-specific to allow
///   disp-specific functionality, or the ghvm builder
///   needs to provide more rules about type construction.

use ghvm;
use std::collections::HashMap;
use super::super::{Token};
use super::{CodegenError};

pub type Production = fn(context: &mut Context, args: &[Token]) -> CodegenResult;
pub type CodegenResult = Result<Object, CodegenError>;

pub struct Block {
    pub locals: HashMap<String, Production>
}

impl Block {
    pub fn new() -> Block {
        let mut block = Block {
            locals: HashMap::new(),
        };
        return block;
    }
}

pub fn unpack(typ: &ghvm::Type, value: i64) -> Token {
    match typ {
        &ghvm::Type::Int => Token::Integer(value),
        _ => Token::None
    }
}

pub struct Context<'a> {
    pub block: Block,
    pub builder: ghvm::FunctionBuilder,
    pub vm: &'a ghvm::VM
}

impl<'a> Context<'a> {
    pub fn new(vm: &'a ghvm::VM) -> Context {
        return Context {
            block: Block::new(),
            builder: ghvm::FunctionBuilder::new(),
            vm: vm
        }
    }
}

pub struct Object {
    pub typ: ghvm::Type, // the type of the register
    pub register: usize // the register containing the value
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

/// thoughts here:
/// - Object probably needs be disp-specific to allow
///   disp-specific functionality, or the ghvm builder
///   needs to provide more rules about type construction.

use std::collections::HashMap;
use super::super::{Token};
use super::{CodegenError};
use warpspeed::{
    BuildObject,
    FunctionBuilder,
    Type,
    VM
};

pub type Production = fn(context: &mut Context, args: &[Token]) -> CodegenResult;
pub type CodegenResult = Result<Object, CodegenError>;

pub struct Block {
    // string w / register
    pub locals: HashMap<String, usize>
}

impl Block {
    pub fn new() -> Block {
        let block = Block {
            locals: HashMap::new(),
        };
        return block;
    }
}

pub struct Context<'a> {
    pub block: Block,
    pub builder: FunctionBuilder,
    pub vm: &'a mut VM
}

impl<'a> Context<'a> {
    pub fn new(vm: &'a mut VM) -> Context {
        return Context {
            block: Block::new(),
            builder: FunctionBuilder::new(),
            vm: vm
        }
    }
}

pub struct Object {
    pub typ: Type, // the type of the register
    pub register: usize // the register containing the value
}

impl Object {
    pub fn from_build_object(build_object: BuildObject) -> Object {
        return Object {
            typ: build_object.typ,
            register: build_object.register
        };
    }

    pub fn to_build_object(&self) -> BuildObject {
        return BuildObject {
            typ: self.typ.clone(),
            register: self.register
        };
    }
}

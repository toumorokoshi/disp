use std::collections::HashMap;
use super::ast::Token;
use super::builtins::{add_builtins};

pub type DFunc = fn(&mut Block, &[Token]) -> Token;

pub struct Block {
    pub locals: HashMap<String, DFunc>
}

impl Block {
    pub fn new() -> Block {
        let mut block = Block {
            locals: HashMap::new(),
        };
        add_builtins(&mut block);
        return block;
    }
}

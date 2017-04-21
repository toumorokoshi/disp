use std::collections::HashMap;
use ghvm;
use super::ast::Token;
use super::builtins::{add_builtins};

pub type DFunc = fn(&mut Block, &[Token]) -> Token;

pub struct Block {
    pub locals: HashMap<String, DFunc>,
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


pub fn unpack(typ: &ghvm::Type, value: i64) -> Token {
    match typ {
        &ghvm::Type::Int => Token::Integer(value),
        _ => Token::None
    }
}

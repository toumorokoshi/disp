#![feature(plugin)]
#![plugin(peg_syntax_ext)]
peg_file! grammar("grammar.rustpeg");
extern crate ghvm;

mod ast;
mod builtins;
mod core;
mod codegen;
mod runtime;

use ast::Token;
use std::{env};
use std::collections::HashMap;
use std::io::{self, Write};
use runtime::{eval};
use codegen::{compile};
use core::{Block};

fn main() {
    let mut block = Block::new();
    let mut vm = ghvm::VM::new();
    loop {
        let inp = read();
        // let result = eval(&mut block, &inp);
        let func = compile(&mut block, &inp);
        let vm_result = vm.execute_function(&func);
        let result = unpack(&func.return_type, vm_result);
        println!("{}", result);
    }
}

fn read() -> Token {
    std::io::stdout().write(b">>> ").unwrap();
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    input = input.replace("\n", "");
    grammar::token(&input).unwrap()
}


pub fn unpack(typ: &ghvm::Type, value: i64) -> Token {
    match typ {
        &ghvm::Type::Int => Token::Integer(value),
        _ => Token::None
    }
}

#![feature(plugin)]
#![plugin(peg_syntax_ext)]
extern crate ghvm;

mod ast;
mod parser;
// mod builtins;
mod codegen;
// mod runtime;

use ast::{Token, ensure_symbol};
use std::{env};
use std::collections::HashMap;
use std::io::{self, Write};
use codegen::{compile};
use parser::{parse};

fn main() {
    let mut vm = ghvm::VM::new();
    loop {
        let inp = read();
        let func = compile(&mut vm, &inp).unwrap();
        let vm_result = func.execute(&mut vm, vec![]);
        let result = unpack(&func.return_type, vm_result);
        println!("{}", result);
        if cfg!(feature = "debug") {
            println!("DEBUG: ops: ");
            func.print_ops();
        }
    }
}

fn read() -> Token {
    std::io::stdout().write(b">>> ").unwrap();
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    input = input.replace("\n", "");
    parse(&input)
}


pub fn unpack(typ: &ghvm::Type, value: i64) -> Token {
    match typ {
        &ghvm::Type::Int => Token::Integer(value),
        &ghvm::Type::Bool => Token::Boolean(if value == 1 {true} else {false}),
        &ghvm::Type::None => Token::None,
        _ => Token::None
    }
}

#![feature(plugin)]
#![plugin(peg_syntax_ext)]
extern crate ghvm;

mod ast;
mod parser;
// mod builtins;
mod codegen;
// mod runtime;

use ast::{Dict, Token, HashableToken, ensure_symbol};
use std::{env};
use std::collections::HashMap;
use std::io::{self, Write};
use std::fs::File;
use std::io::prelude::*;
use codegen::{compile};
use parser::{parse};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => execute(&args[1]),
        _ => repl()
    }
}

fn repl() {
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

fn execute(path: &str) {
    let mut vm = ghvm::VM::new();
    let mut file = File::open(path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let inp = parse(&input);
    let func = compile(&mut vm, &inp).unwrap();
    let vm_result = func.execute(&mut vm, vec![]);
    let result = unpack(&func.return_type, vm_result);
    println!("{}", result);
    if cfg!(feature = "debug") {
        println!("DEBUG: ops: ");
        func.print_ops();
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

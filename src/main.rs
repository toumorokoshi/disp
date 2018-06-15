#![feature(plugin)]
#![plugin(peg_syntax_ext)]
extern crate warpspeed;

mod ast;
mod parser;
mod codegen;
mod vm;

use ast::{Dict, Token, HashableToken};
use std::{env};
use std::io::{self, Write};
use std::fs::File;
use std::io::prelude::*;
use codegen::{compile};
use parser::{parse};
use warpspeed::{Type, VM};
use vm::build_vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => execute(&args[1]),
        _ => repl()
    }
}

fn repl() {
    let mut vm = build_vm();
    loop {
        let inp = read();
        let func = compile(&mut vm, &inp).unwrap();
        let vm_result = func.execute(&vm.handle(), vec![]);
        let result = unpack(&func.return_type, vm_result);
        println!("{}", result);
        if cfg!(feature = "debug") {
            println!("DEBUG: ops: ");
            func.print_ops();
        }
   }
}

fn execute(path: &str) {
    let mut vm = build_vm();
    let mut file = File::open(path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let inp = parse(&input);
    let func = compile(&mut vm, &inp).unwrap();
    if cfg!(feature = "debug") {
        println!("DEBUG: ops: ");
        func.print_ops();
    }
    let vm_result = func.execute(&vm.handle(), vec![]);
    let result = unpack(&func.return_type, vm_result);
    println!("{}", result);
}

fn read() -> Token {
    std::io::stdout().write(b">>> ").unwrap();
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    input = input.replace("\n", "");
    parse(&input)
}


pub fn unpack(typ: &Type, value: i64) -> Token {
    match typ {
        &Type::Int => Token::Integer(value),
        &Type::Bool => Token::Boolean(if value == 1 {true} else {false}),
        &Type::None => Token::None,
        _ => Token::None
    }
}

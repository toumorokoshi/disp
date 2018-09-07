#![feature(plugin)]
extern crate warpspeed;
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod parser;
mod codegen;
mod vm;

use ast::{Dict, Token, HashableToken};
use std::{
    env,
    sync::Arc,
    time::Duration,
    thread::sleep,
};
use std::io::{self, Write};
use std::fs::File;
use std::io::prelude::*;
use codegen::{compile};
use parser::{full_parse};
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
        let func = Arc::new(compile(&mut vm, &inp).unwrap());
        if cfg!(feature = "debug") {
            println!("DEBUG: ops: ");
            func.print_ops();
        }
        vm.submit(func.clone(), vec![]);
        sleep(Duration::from_millis(50));
   }
}

fn execute(path: &str) {
    let mut vm = build_vm();
    let mut file = File::open(path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let inp = full_parse(&input);
    let func = compile(&mut vm, &inp).unwrap();
    if cfg!(feature = "debug") {
        println!("DEBUG: ops: ");
        func.print_ops();
    }
    vm.submit(Arc::new(func), vec![])
}

fn read() -> Token {
    std::io::stdout().write(b">>> ").unwrap();
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    input = input.replace("\n", "");
    parse_with_print(&input)
}


pub fn unpack(typ: &Type, value: i64) -> Token {
    match typ {
        &Type::Int => Token::Integer(value),
        &Type::Bool => Token::Boolean(if value == 1 {true} else {false}),
        &Type::None => Token::None,
        _ => Token::None
    }
}

/// Parse the body in question, and wrap in a print statement
fn parse_with_print(body: &str) -> Token {
    let input = full_parse(&body);
    Token::Expression(vec![
        Token::Symbol(Box::new(String::from("print"))),
        input
    ])
}

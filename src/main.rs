#![feature(plugin)]
#![plugin(peg_syntax_ext)]
peg_file! grammar("grammar.rustpeg");

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
use core::{Block};

fn main() {
    let mut block = Block::new();
    loop {
        let inp = read();
        let result = eval(&mut block, &inp);
        // let module = compile(&mut block, inp);
        // let result = vm.execute(module)
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

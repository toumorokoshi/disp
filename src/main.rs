#![feature(plugin)]
#![plugin(peg_syntax_ext)]
peg_file! grammar("grammar.rustpeg");

mod ast;

use ast::Token;
use std::{env};
use std::collections::HashMap;
use std::io::{self, Write};
mod runtime;
use runtime::{eval, eval_expr, Block};

fn main() {
    let mut block = Block::new();
    loop {
        let inp = read();
        let result = eval(&mut block, &inp);
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

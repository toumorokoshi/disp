#![feature(plugin)]
#![plugin(peg_syntax_ext)]
peg_file! grammar("grammar.rustpeg");

mod ast;

use std::{env};
use std::collections::HashMap;
use std::io::{self, Write};
mod runtime;
use runtime::{eval_expr, Block};

fn main() {
    let mut block = Block::new();
    loop {
        let inp = read();
        let result = eval_expr(&mut block, &inp);
        print(result);
    }
}

fn read() -> Vec<Token> {
    std::io::stdout().write(b">>> ").unwrap();
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    grammar::token_list(&input)
}

fn print(values: Vec<String>) {
    for v in values {
        println!("{}", v);
    }
}

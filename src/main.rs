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

fn read() -> Vec<String> {
    std::io::stdout().write(b">>> ").unwrap();
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    input.push(' ');
    let mut result = Vec::new();
    for token in input.split(" ") {
        result.push(String::from(token));
    }
    return result;
}

fn print(values: Vec<String>) {
    for v in values {
        println!("{}", v);
    }
}

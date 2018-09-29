#![feature(plugin)]
extern crate pest;
extern crate warpspeed;
#[macro_use]
extern crate pest_derive;
extern crate libc;
extern crate llvm_sys;

mod ast;
mod codegen;
mod error;
mod llvm_builder;
mod llvm_codegen;
mod loader;
mod parser;
mod stdlib;
mod vm;

use ast::{HashableToken, Token};
use codegen::compile;
use error::DispError;
// Exporting all functions publicy, so they will
// be discovered by llvm.
pub use llvm_codegen::{compile_module, native_functions::*, Compiler};
use loader::exec_file;
use parser::full_parse;
use std::{
    env,
    io::{self, Write},
    sync::Arc,
    thread::sleep,
    time::Duration,
};
// use stdlib::load_stdlib;
use vm::build_vm;
use warpspeed::Type;

fn main() {
    // let builder = LLVMBuilder::new();
    // builder.build_function();
    // builder.run();
    // builder.cleanup();
    let args: Vec<String> = env::args().collect();
    let result = match args.len() {
        2 => execute(&args[1]),
        _ => repl(),
    };
    if let Err(ref message) = result {
        panic!("{}", message);
    }
}

fn repl() -> Result<(), DispError> {
    let mut vm = build_vm()?;
    loop {
        let inp = read()?;
        let func = Arc::new(compile(&mut vm, &inp)?);
        if cfg!(feature = "debug") {
            println!("DEBUG: ops: ");
            func.print_ops();
        }
        vm.submit(func.clone(), vec![]);
        sleep(Duration::from_millis(1000));
    }
}

fn execute(path: &str) -> Result<(), DispError> {
    // let mut vm = build_vm()?;
    exec_file(path)?;
    // vm.shutdown_on_idle();
    Ok(())
}

fn read() -> Result<Token, DispError> {
    std::io::stdout().write(b">>> ")?;
    std::io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.replace("\n", "");
    Ok(parse_with_print(&input))
}

pub fn unpack(typ: &Type, value: i64) -> Token {
    match typ {
        &Type::Int => Token::Integer(value),
        &Type::Bool => Token::Boolean(if value == 1 { true } else { false }),
        &Type::None => Token::None,
        _ => Token::None,
    }
}

/// Parse the body in question, and wrap in a print statement
fn parse_with_print(body: &str) -> Token {
    let input = full_parse(&body);
    Token::Expression(vec![
        Token::Symbol(Box::new(String::from("println"))),
        input,
    ])
}

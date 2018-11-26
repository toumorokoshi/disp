#![feature(plugin)]
#![feature(duration_float)]
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate inference;
extern crate libc;
extern crate llvm_sys;

mod ast;
mod error;
mod llvm_builder;
mod llvm_codegen;
mod loader;
mod parser;
mod stdlib;

use ast::Token;
use error::{DispError, DispResult, GenericError, GenericResult};
// Exporting all functions publicy, so they will
// be discovered by llvm.
use llvm_builder::{Builder, LLVMInstruction};
pub use llvm_codegen::{
    compile_module, native_functions::*, Compiler, CompilerData, Function, FunctionType,
    LLVMFunction, NativeFunction, Scope,
};
use loader::{exec_file, load_file};
use parser::parse;
use std::{
    env,
    io::{self, Write},
};
use stdlib::load_stdlib;
// use stdlib::load_stdlib;

fn main() {
    // let builder = LLVMBuilder::new();
    // builder.build_function();
    // builder.run();
    // builder.cleanup();
    let args: Vec<String> = env::args().collect();
    let result = match args.len() {
        2 => execute(&args[1]),
        _ => {panic!("no repl atm.")}
        // _ => repl(),
    };
    if let Err(ref message) = result {
        panic!("{}", message);
    }
}

// fn repl() -> Result<(), DispError> {
//     let mut vm = build_vm()?;
//     loop {
//         let inp = read()?;
//         let func = Arc::new(compile(&mut vm, &inp)?);
//         if cfg!(feature = "debug") {
//             println!("DEBUG: ops: ");
//             func.print_ops();
//         }
//         vm.submit(func.clone(), vec![]);
//         sleep(Duration::from_millis(1000));
//     }
// }

fn execute(path: &str) -> Result<(), GenericError> {
    let mut compiler = Compiler::new();
    {
        load_stdlib(&mut compiler)?;
    }
    {
        exec_file(&mut compiler, path)?;
    }
    Ok(())
}

fn read() -> Result<Token, GenericError> {
    std::io::stdout().write(b">>> ")?;
    std::io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.replace("\n", "");
    Ok(parse_with_print(&input))
}

/// Parse the body in question, and wrap in a print statement
fn parse_with_print(body: &str) -> Token {
    let input = parse(&body);
    Token::Expression(vec![
        Token::Symbol(Box::new(String::from("println"))),
        input,
    ])
}

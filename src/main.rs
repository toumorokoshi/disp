#![feature(plugin)]
#![feature(duration_float)]
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate inference;
extern crate libc;
extern crate llvm_sys;

mod ast;
mod compiler;
mod error;
mod expressions;
mod function_loader;
mod llvm_builder;
mod llvm_codegen;
mod loader;
mod macros;
mod parser;
mod stdlib;
mod type_annotator;

use ast::Token;
use compiler::compile;
use error::{DispError, DispResult, GenericError, GenericResult};
// Exporting all functions publicy, so they will
// be discovered by llvm.
use expressions::{get_builtin_expressions, BuiltinExpressions};
use function_loader::{parse_functions_and_macros, FunctionMap, UnparsedFunction};
use llvm_builder::{Builder, LLVMInstruction};
pub use llvm_codegen::{
    build_functions, compile_module, gen_token, native_functions::*, CodegenError, Compiler,
    CompilerData, Function, FunctionType, LLVMFunction, NativeFunction, Object, Scope, Type,
};
use loader::{exec_file, load_file};
use macros::{apply_macros_to_function_map, Macro, MacroMap};
use parser::parse;
use std::{
    env,
    fs::File,
    io::{self, Read, Write},
};
use stdlib::load_stdlib;
use type_annotator::{annotate_types, AnnotatedFunction, AnnotatedFunctionMap, TypevarFunction};
// use stdlib::load_stdlib;

fn main() {
    // let builder = LLVMBuilder::new();
    // builder.build_function();
    // builder.run();
    // builder.cleanup();
    let args: Vec<String> = env::args().collect();
    let result = match args.len() {
        2 => execute_2(&args[1]),
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

fn execute_2(path: &str) -> Result<(), GenericError> {
        let mut compiler = Compiler::new();
        let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    compile(&mut compiler, &input)?;
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

#![feature(plugin)]
#![feature(duration_float)]
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate inference;
extern crate libc;
extern crate llvm_sys;

mod array;
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
mod workflow;

use self::array::llvm_declare_array;
use self::ast::Token;
use self::compiler::{CompilerData, Type};
use self::error::{DispError, DispResult, GenericError, GenericResult};
// Exporting all functions publicy, so they will
// be discovered by llvm.
use self::expressions::{get_builtin_expressions, BuiltinExpressions};
use self::function_loader::{parse_functions_and_macros, FunctionMap, UnparsedFunction};
use self::llvm_builder::{Builder, LLVMInstruction};
pub use self::llvm_codegen::{
    build_functions, to_ptr, CodegenError, Compiler, Function, FunctionType, NativeFunction,
    Object, Scope, to_llvm_type,
};
use self::loader::{exec_file, load_file};
use self::macros::{apply_macros_to_function_map, parse_macro, MacroMap};
use self::parser::parse;
use self::stdlib::{load_stdlib, LIB_FILE};
use self::type_annotator::{
    annotate_types, AnnotatedFunction, AnnotatedFunctionMap, TypevarFunction,
};
use self::workflow::load_string_into_compiler;
use std::{
    env,
    fs::File,
    io::{self, Read, Write},
};
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

fn execute(path: &str) -> Result<(), GenericError> {
    let mut compiler = Compiler::new();
    let mut input = String::new();
    // load the standard lib
    let mut stdlib = File::open(LIB_FILE)?;
    stdlib.read_to_string(&mut input)?;
    // load the main file
    let mut file = File::open(path)?;
    file.read_to_string(&mut input)?;
    load_string_into_compiler(&mut compiler, &input)?;
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

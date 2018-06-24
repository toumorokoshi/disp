#![feature(test)]
extern crate test;
extern crate futures;
extern crate nix;
extern crate num_cpus;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate spmc;
extern crate tokio;

mod benchmark;
mod builder;
mod core;
mod function;
mod fiber;
mod ops;
mod runtime;
mod types;
mod vm;

pub use builder::{BuildObject, FunctionBuilder};
pub use core::{Register, RegisterList, Value, ValueList};
pub use fiber::{Fiber};
pub use function::{Function, VMFunction, NativeFunction};
pub use ops::{Op, OpList};
pub use runtime::Runtime;
pub use types::{Type};
pub use vm::VM;


#[macro_use]
extern crate serde_derive;

fn main() {
    let mut vm_instance = vm::VM::new();
    vm_instance.wait();
}

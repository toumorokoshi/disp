#![feature(test)]
extern crate test;
extern crate futures;
extern crate nix;
extern crate num_cpus;
extern crate serde;
extern crate serde_json;
extern crate spmc;
extern crate tokio;

mod benchmark;
mod bytecode;
mod core;
mod fiber;
mod vm;

pub(crate) use bytecode::{Op};
pub(crate) use core::{Register, RegisterList, Value, ValueList};
pub(crate) use fiber::{Fiber};

#[macro_use]
extern crate serde_derive;

fn main() {
    let mut vm_instance = vm::VM::new();
    vm_instance.wait();
}

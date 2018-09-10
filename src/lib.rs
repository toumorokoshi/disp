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
mod heap;
mod ops;
mod runtime;
mod worker;
mod worker_heap;
mod types;
mod vm;
mod vm_handle;

pub use builder::{BuildObject, FunctionBuilder};
pub use core::{Register, RegisterList, Value, ValueList};
pub use fiber::{Fiber};
pub use function::{VMFunction, NativeFunction};
pub use heap::{Heap};
pub use ops::{Op, OpList};
pub use runtime::Runtime;
pub use worker::{Worker};
pub use worker_heap::{WorkerHeap, WORKER_HEAP};
pub use types::{Type};
pub use vm::VM;
pub use vm_handle::VMHandle;


#[macro_use]
extern crate serde_derive;

// fn main() {
//     let mut vm_instance = vm::VM::new();
//     vm_instance.wait();
// }

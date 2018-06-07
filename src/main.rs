#![feature(test)]
extern crate test;
extern crate futures;
extern crate nix;
extern crate num_cpus;
extern crate serde;
extern crate serde_json;
extern crate spmc;

mod benchmark;
mod vm;

#[macro_use]
extern crate serde_derive;

fn main() {
    let vm_instance = vm::VM::new();
}


#[derive(Serialize, Deserialize)]
pub struct User {
    age: u8,
    name: String
}

/// contains built in functions
use warpspeed::{
    VMHandle,
    WorkerHeap,
    Value,
    ValueList
};
use std::io::{self, Read};


pub fn print(vm: &VMHandle, heap: &mut WorkerHeap, args: &mut ValueList) -> Value {
    println!("{0}", args[0]);
    0
}


pub fn read_line(vm: &VMHandle, heap: &mut WorkerHeap, args: &mut ValueList) -> Value {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    heap.strings.insert(0, input.clone());
    println!("{}", input);
    0
}

// NOTE: this method currently performs and implicit lock
// via a mutex (within Rust's IO object). It might make sense to
// eliminated this at some point.
// pub fn read_line(args: &mut ValueList) -> Value {
//     let mut buffer = String::new();
//     io::stdin().read_to_string(&mut buffer).unwrap();
// }

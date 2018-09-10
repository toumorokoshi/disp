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
    return 0
}


pub fn print_string(vm: &VMHandle, heap: &mut WorkerHeap, args: &mut ValueList) -> Value {
    println!("{0}", heap.strings[args[0] as usize]);
    return 0
}


pub fn read_line(vm: &VMHandle, heap: &mut WorkerHeap, args: &mut ValueList) -> Value {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    heap.strings.push(input.clone());
    return (heap.strings.len() - 1) as Value;
}

// NOTE: this method currently performs and implicit lock
// via a mutex (within Rust's IO object). It might make sense to
// eliminated this at some point.
// pub fn read_line(args: &mut ValueList) -> Value {
//     let mut buffer = String::new();
//     io::stdin().read_to_string(&mut buffer).unwrap();
// }

/// contains built in functions
use warpspeed::{
    VMHandle,
    WorkerHeap,
    Value,
    ValueList
};
use std::io::{self, Read};


/// Cast a string to an integer
pub fn int(vm: &VMHandle, heap: &mut WorkerHeap, args: &mut ValueList) -> Value {
    let int_as_string = heap.strings[args[0] as usize].clone();
    println!("{}", &int_as_string);
    return int_as_string.parse::<i64>().unwrap();
}


pub fn print(vm: &VMHandle, heap: &mut WorkerHeap, args: &mut ValueList) -> Value {
    println!("{0}", args[0]);
    return 0
}


pub fn print_string(vm: &VMHandle, heap: &mut WorkerHeap, args: &mut ValueList) -> Value {
    println!("{0}", heap.strings[args[0] as usize]);
    return 0
}


// NOTE: this method currently performs and implicit lock
// via a mutex (within Rust's IO object). It might make sense to
// eliminated this at some point.
pub fn read_line(vm: &VMHandle, heap: &mut WorkerHeap, args: &mut ValueList) -> Value {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            input.pop();
            heap.strings.push(input);
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
    return (heap.strings.len() - 1) as Value;
}

pub fn set(vm: &VMHandle, heap: &mut WorkerHeap, args: &mut ValueList) -> Value {
    0
}

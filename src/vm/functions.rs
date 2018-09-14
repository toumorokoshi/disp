/// contains built in functions
use warpspeed::{
    VMHandle,
    WorkerHeap,
    Value,
    ValueList
};
use std::io::{self};


/// Cast a string to an integer
pub fn int(_vm: &VMHandle, heap: &mut WorkerHeap, args: &mut ValueList) -> Value {
    let int_as_string = heap.strings[args[0] as usize].clone();
    return int_as_string.parse::<i64>().unwrap();
}


pub fn print(_vm: &VMHandle, _heap: &mut WorkerHeap, args: &mut ValueList) -> Value {
    println!("{0}", args[0]);
    return 0
}


pub fn print_string(_vm: &VMHandle, heap: &mut WorkerHeap, args: &mut ValueList) -> Value {
    println!("{0}", heap.strings[args[0] as usize]);
    return 0
}


// NOTE: this method currently performs and implicit lock
// via a mutex (within Rust's IO object). It might make sense to
// eliminated this at some point.
pub fn read_line(_vm: &VMHandle, heap: &mut WorkerHeap, _args: &mut ValueList) -> Value {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            input.pop();
            heap.strings.push(input);
            return (heap.strings.len() - 1) as Value;
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
}


/// add an entry. This will eventually be for multiple different
/// types, but it only works for maps for now.
pub fn add(_vm: &VMHandle, heap: &mut WorkerHeap, args: &mut ValueList) -> Value {
    println!("called");
    let map = &mut heap.maps[args[0] as usize];
    map.insert(args[1], args[2]);
    0
}


/// returns the count of the collection in question
/// (only works for maps for now)
pub fn count(_vm: &VMHandle, heap: &mut WorkerHeap, args: &mut ValueList) -> Value {
    let map = &heap.maps[args[0] as usize];
    map.len() as Value
}

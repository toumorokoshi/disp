mod functions;

use std::{sync::Arc, thread::sleep, time::Duration};

use self::functions::{add, count, int, print, print_string, println, read_line};
use super::DispError;
use warpspeed::{Type, VM};

/// build a specialized VM for disp, containing
/// some builtins
pub fn build_vm() -> Result<VM, DispError> {
    let mut vm = VM::new();
    match Arc::get_mut(&mut vm.heap) {
        Some(heap) => {
            heap.add_native_func(
                String::from("add"),
                vec![
                    Type::Map(Box::new(Type::String), Box::new(Type::Bool)),
                    Type::String,
                    Type::Bool,
                ],
                Type::Int,
                add,
            );
            heap.add_native_func(
                String::from("count"),
                vec![Type::Map(Box::new(Type::String), Box::new(Type::Bool))],
                Type::Int,
                count,
            );
            heap.add_native_func(String::from("print"), vec![Type::Int], Type::None, print);
            heap.add_native_func(
                String::from("print"),
                vec![Type::String],
                Type::None,
                print_string,
            );
            heap.add_native_func(
                String::from("println"),
                vec![Type::Int],
                Type::None,
                println,
            );
            heap.add_native_func(String::from("read-line"), vec![], Type::String, read_line);
            heap.add_native_func(String::from("Int"), vec![Type::String], Type::Int, int);
        }
        None => {
            panic!("unable to get mutable handle to vm heap during initialization.");
        }
    }
    sleep(Duration::from_millis(1000));
    Ok(vm)
}

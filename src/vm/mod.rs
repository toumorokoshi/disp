use std::sync::Arc;
use warpspeed::{VM, NativeFunction, Type};
mod functions;
use self::functions::{
    add,
    count,
    int,
    print,
    print_string,
    println,
    read_line,
};

/// build a specialized VM for disp, containing
/// some builtins
pub fn build_vm() -> VM {
    let mut vm = VM::new();
    match Arc::get_mut(&mut vm.heap) {
        Some(heap) => {
            heap.add_native_func(
                String::from("add"),
                vec![Type::Map(Box::new(Type::String), Box::new(Type::Bool)), Type::String, Type::Bool],
                Type::Int,
                add
            );
            heap.add_native_func(
                String::from("count"),
                vec![Type::Map(Box::new(Type::String), Box::new(Type::Bool))],
                Type::Int,
                count
            );
            heap.add_native_func(
                String::from("print"),
                vec![Type::Int],
                Type::None,
                print
            );
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
                println
            );
            heap.add_native_func(
                String::from("read-line"),
                vec![],
                Type::String,
                read_line,
            );
            heap.add_native_func(
                String::from("Int"),
                vec![Type::String],
                Type::Int,
                int,
            );
        },
        None => { panic!("unable to warmup vm");}
    }
    return vm;
}

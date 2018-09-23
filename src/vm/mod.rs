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
            heap.functions_native.insert(String::from("add"), Arc::new(NativeFunction{
                registers: vec![Type::Int, Type::Int],
                return_type: Type::Int,
                func: add
            }));
            heap.functions_native.insert(String::from("count"), Arc::new(NativeFunction{
                registers: vec![Type::Map(Box::new(Type::String), Box::new(Type::Bool))],
                return_type: Type::Int,
                func: count
            }));
            heap.functions_native.insert(String::from("print"), Arc::new(NativeFunction{
                registers: vec![Type::Int],
                return_type: Type::None,
                func: print
            }));
            heap.functions_native.insert(String::from("println"), Arc::new(NativeFunction{
                registers: vec![Type::Int],
                return_type: Type::None,
                func: println
            }));
            heap.functions_native.insert(String::from("print-string"), Arc::new(NativeFunction{
                registers: vec![Type::Int],
                return_type: Type::None,
                func: print_string,
            }));
            heap.functions_native.insert(String::from("read-line"), Arc::new(NativeFunction{
                registers: vec![],
                return_type: Type::String,
                func: read_line,
            }));
            heap.functions_native.insert(String::from("Int"), Arc::new(NativeFunction{
                registers: vec![Type::String],
                return_type: Type::Int,
                func: int,
            }));
        },
        None => { panic!("unable to warmup vm");}
    }
    return vm;
}

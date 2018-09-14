use std::sync::Arc;
use warpspeed::{VM};
mod functions;
use self::functions::{
    add,
    count,
    int,
    print,
    print_string,
    read_line,
};

/// build a specialized VM for disp, containing
/// some builtins
pub fn build_vm() -> VM {
    let mut vm = VM::new();
    match Arc::get_mut(&mut vm.heap) {
        Some(heap) => {
            heap.functions_native.insert(String::from("add"), Arc::new(add));
            heap.functions_native.insert(String::from("count"), Arc::new(count));
            heap.functions_native.insert(String::from("print"), Arc::new(print));
            heap.functions_native.insert(String::from("print-string"), Arc::new(print_string));
            heap.functions_native.insert(String::from("read-line"), Arc::new(read_line));
            heap.functions_native.insert(String::from("Int"), Arc::new(int));
        },
        None => { panic!("unable to warmup vm");}
    }
    return vm;
}

use std::sync::Arc;
use warpspeed::{VM, VMFunction};
mod functions;
use self::functions::{print};

/// build a specialized VM for disp, containing
/// some builtins
pub fn build_vm() -> VM {
    let mut vm = VM::new();
    match Arc::get_mut(&mut vm.heap) {
        Some(heap) => {
            heap.functions_native.insert(String::from("print"), Arc::new(print));
        },
        None => { panic!("unable to warmup vm");}
    }
    return vm;
}

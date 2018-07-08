use std::sync::Arc;
use super::{VMFunction, NativeFunction};

/// The heap represents data and objects
/// that are shared across workers in the VM.
/// Separate from the top-level VM because
/// this must be passed around workers.
pub struct Heap {
    pub functions_native: Vec<Arc<VMFunction>>,
    pub functions_vm: Vec<Arc<NativeFunction>>
}

impl Heap {
    pub fn new() -> Heap {
        return Heap {
            functions_native: Vec::new(),
            functions_vm: Vec::new(),
        };
    }
}

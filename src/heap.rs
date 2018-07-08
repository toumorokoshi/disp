use std::sync::Arc;
use std::collections::HashMap;
use super::{VMFunction, NativeFunction};

/// The heap represents data and objects
/// that are shared across workers in the VM.
/// Separate from the top-level VM because
/// this must be passed around workers.
pub struct Heap {
    pub functions_native: HashMap<String, Arc<NativeFunction>>,
    pub functions_vm: HashMap<String, Arc<VMFunction>>
}

impl Heap {
    pub fn new() -> Heap {
        return Heap {
            functions_native: HashMap::new(),
            functions_vm: HashMap::new()
        };
    }
}

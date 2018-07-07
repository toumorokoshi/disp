use super::{Function};

/// The heap represents data and objects
/// that are shared across workers in the VM.
/// Separate from the top-level VM because
/// this must be passed around workers.
pub struct Heap {
    functions: Vec<Function>
}

impl Heap {
    pub fn new() -> Heap {
        return Heap {
            functions: Vec::new()
        };
    }
}

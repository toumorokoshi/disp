use std::sync::Arc;
use super::{Heap};


/// The VM Handle is passed around from
/// worker to worker. it is an interface
/// that can be shared across multiple threads.
pub struct VMHandle {
    pub heap: Arc<Heap>
}

impl VMHandle {
    pub fn new(heap: Arc<Heap>) -> VMHandle {
        return VMHandle {
            heap: heap
        };
    }
}

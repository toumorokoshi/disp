use tokio::runtime::current_thread::{Handle, Runtime};
use super::{WorkerHeap};


/// Workers handle the execution of Fibers,
/// and multiple can be associated with a single VM process.
/// workers and it's structured cannot cross thread boundaries:
/// it is intended to be owned by a single thread for it's lifetime.
pub struct Worker {
    runtime: Runtime,
    heap: Rc<WorkerHeap>
}

/// WorkerHandles can
pub struct WorkerHandle {
}

impl Worker {
    pub fn new() -> Worker {
        return Worker {
            runtime: Runtime::new().unwrap(),
            heap: Rc::new(WorkerHeap::new())
        };
    }
}

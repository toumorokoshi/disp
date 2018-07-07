use std::sync::Arc;
use super::{
    Heap,
    Fiber,
    Runtime,
    VMHandle,
};



pub struct VM {
    // the runtime handles the execution of
    // fibers, and the spawning of 1 per worker
    runtime: Runtime,
    // the heap contains all data (functions, constants, etc)
    heap: Arc<Heap>,
}

impl VM {
    pub fn new() -> VM {
        return VM { runtime: Runtime::new(), heap: Arc::new(Heap::new()),
        };
    }

    pub fn wait(&mut self) {
        self.runtime.shutdown_on_idle();
    }

    // submit a fiber for execution
    pub fn submit(&mut self, fiber: Fiber) {
        self.runtime.submit(fiber);
    }

    // return a handle to the VM. Rather than
    // pass around the VM directly, the VM
    // should expose functions in the handle that
    // can modify the VM.
    pub fn handle(&self) -> VMHandle {
        return VMHandle::new(self.heap.clone());
    }
}

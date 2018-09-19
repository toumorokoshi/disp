use std::sync::Arc;
use super::{
    Heap,
    Fiber,
    Runtime,
    ValueList,
    VMFunction,
    VMHandle,
};



pub struct VM {
    // the runtime handles the execution of
    // fibers, and the spawning of 1 per worker
    runtime: Runtime,
    // the heap contains all data (functions, constants, etc)
    pub heap: Arc<Heap>,
}

impl VM {
    pub fn new() -> VM {
        return VM { runtime: Runtime::new(), heap: Arc::new(Heap::new()),
        };
    }

    /// submit a function for execution.
    pub fn submit(&mut self, function: Arc<VMFunction>, args: ValueList) {
        let worker_id = self.runtime.random_worker();
        let ref worker = self.runtime.pool.workers[worker_id];
        let fiber = Fiber::new(
            function.clone(),
            self.handle()
        );
        worker.runtime.spawn(fiber).unwrap();
    }

    // return a handle to the VM. Rather than
    // pass around the VM directly, the VM
    // should expose functions in the handle that
    // can modify the VM.
    pub fn handle(&self) -> VMHandle {
        return VMHandle::new(self.heap.clone());
    }

    pub fn shutdown_on_idle(self) {
        self.runtime.shutdown_on_idle();
    }
}

use num_cpus;
use tokio::runtime::Runtime;
use futures::{Future, Async};
use super::{ValueList, Fiber, Op};



pub struct VM {
    tokio_runtime: Runtime
}

impl VM {
    pub fn new() -> VM {
        // tokio handles a lot of the complexity around
        // managing a worker per thread, and providing
        // apis to submit tasks to them.
        let mut runtime = Runtime::new().unwrap();
        let registers = ValueList::new();
        let ops = vec![];
        runtime.spawn(Fiber::new(registers, ops));
        // TODO: spawn one worker per thread.
        // TODO: thread pin.
        // NO easy way to get spawned threads.
        // would be nice if the pool had a handle to it, consider contributing?
        // we spawn futures here.
        // TODO:
        // for now we can force spawn threads, via spawn_worker
        // then we can submit the task to the worker specifically,
        // via getting the entry from the worker list in pool,
        // then calling submit_external.
        return VM {
            tokio_runtime: runtime
        };
    }

    pub fn wait(mut self) {
        self.tokio_runtime.shutdown_on_idle().wait().unwrap();
    }
}

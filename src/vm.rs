use num_cpus;
use futures::{Future, Async};
use super::{ValueList, Fiber, Op, Runtime};



pub struct VM {
    runtime: Runtime
}

impl VM {
    pub fn new() -> VM {
        // tokio handles a lot of the complexity around
        // managing a worker per thread, and providing
        // apis to submit tasks to them.
        let mut runtime = Runtime::new();
        // runtime.spawn(Fiber::new(registers, ops));
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
            runtime: runtime
        };
    }

    pub fn wait(&mut self) {
        self.runtime.shutdown_on_idle();
    }

    // submit a fiber for execution
    pub fn submit(&mut self, fiber: Fiber) {
        self.runtime.submit(fiber);
    }
}

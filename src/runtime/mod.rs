mod pool;
use rand;
use std::sync::Arc;
use super::{Fiber, VMFunction};
pub use self::pool::WorkerPool;


pub struct Runtime {
    pool: WorkerPool,
}

impl Runtime {
    pub fn new() -> Runtime {
        return Runtime {
            pool: WorkerPool::new()
        }
    }

    /// submit a function to a random worker
    pub fn submit(&self, function: Arc<VMFunction>) {
        let worker_id = rand::random::<usize>() % self.pool.len();
        self.submit_to_worker(worker_id, function);
    }

    /// submit a function to a specific worker
    pub fn submit_to_worker(&self, worker_id: usize, function: Arc<VMFunction>) {
        let ref worker = self.pool.workers[worker_id];
        let fiber = Fiber::new(
            function: function.clone(),
        );
        worker.runtime.spawn(fiber).unwrap()
    }

    pub fn shutdown_on_idle(&self) {
    }
}

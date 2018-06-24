mod pool;

use rand;
use super::{Fiber};
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

    /// submit work to a worker, at random.
    pub fn submit(&self, fiber: Fiber) {
        let worker_id = rand::random::<usize>() % self.pool.len();
        self.submit_to_worker(worker_id, fiber)
    }

    /// submit work to a specific worker
    pub fn submit_to_worker(&self, worker_id: usize, fiber: Fiber) {
        let ref worker = self.pool.workers[worker_id];
        worker.runtime.spawn(fiber).unwrap()
    }

    pub fn shutdown_on_idle(&self) {
    }
}

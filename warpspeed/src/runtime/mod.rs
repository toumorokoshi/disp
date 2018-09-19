mod pool;

use rand;
use super::{Fiber};
pub use self::pool::WorkerPool;


pub struct Runtime {
    pub pool: WorkerPool,
}

impl Runtime {
    pub fn new() -> Runtime {
        return Runtime {
            pool: WorkerPool::new()
        }
    }

    pub fn random_worker(&self) -> usize {
        rand::random::<usize>() % self.pool.len()
    }

    pub fn shutdown_on_idle(self) {
        for i in 0..self.pool.len() {
            self.pool.workers[i].shutdown();
        }
        for worker in self.pool.workers {
            worker.thread.join();
        }
    }
}

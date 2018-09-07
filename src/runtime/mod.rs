mod pool;
use rand;
use std::sync::Arc;
use super::{Fiber, VMFunction};
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

    /// return a random, valid worker id
    pub fn random_worker(&self) -> usize {
        rand::random::<usize>() % self.pool.len()
    }

    pub fn shutdown_on_idle(&self) {
    }
}

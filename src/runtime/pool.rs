use num_cpus;
use std::vec::Vec;
use super::super::{Worker};

/// The pool is responsible for owning the reactors
/// themselves.
pub struct WorkerPool {
    pub workers: Vec<Worker>
}

impl WorkerPool {
    pub fn new() -> WorkerPool {
        let num = num_cpus::get();
        let mut workers = Vec::with_capacity(num);
        for i in 0..num {
            workers.push(Worker::spawn(i));
        }
        return WorkerPool {
            workers: workers
        };
    }

    pub fn len(&self) -> usize {
        return self.workers.len();
    }
}

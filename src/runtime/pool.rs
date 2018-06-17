use num_cpus;
use std::{thread};
use tokio::runtime::;

/// The pool is responsible for owning the reactors
/// themselves.
struct Pool {
}

impl Pool {
    pub fn new() -> Pool {
        let num = num_cpus::get();
        let workers = Vec::with_capacity(num);
        for i in 0..num {
            workers.push(thread.spawn(move || {
            });
        }
        return Pool {
        };
    }
}

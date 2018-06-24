use num_cpus;
use std::{
    thread::{spawn, JoinHandle},
    sync::mpsc::channel,
    vec::Vec
};
use tokio::runtime::current_thread::{Handle, Runtime};

/// The pool is responsible for owning the reactors
/// themselves.
pub struct WorkerPool {
    pub workers: Vec<WorkerHandle>
}

pub struct WorkerHandle {
    pub thread: JoinHandle<()>,
    pub runtime: Handle
}

impl WorkerPool {
    pub fn new() -> WorkerPool {
        let num = num_cpus::get();
        let mut workers = Vec::with_capacity(num);
        for _ in 0..num {
            let (tx, rx) = channel();
            let thread_handle = spawn(move || {
                let mut runtime = Runtime::new().unwrap();
                {
                    tx.send(runtime.handle()).unwrap();
                }
                runtime.run().unwrap()
            });
            let runtime_handle = rx.recv().unwrap();
            workers.push(WorkerHandle {
                thread: thread_handle,
                runtime: runtime_handle
            });
        }
        return WorkerPool {
            workers: workers
        };
    }

    pub fn len(&self) -> usize {
        return self.workers.len();
    }
}

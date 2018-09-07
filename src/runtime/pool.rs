use num_cpus;
use nix::{
    sched::{sched_setaffinity, CpuSet},
    unistd::Pid,
};
use std::{
    rc::Rc,
    sync::mpsc::channel,
    thread::{spawn, JoinHandle},
    vec::Vec
};
use tokio::runtime::current_thread::{Handle, Runtime};
use super::super::{WorkerHeap};

/// The pool is responsible for owning the reactors
/// themselves.
pub struct WorkerPool {
    pub workers: Vec<WorkerHandle>
}

pub struct WorkerHandle {
    pub thread: JoinHandle<()>,
    pub runtime: Handle,
    pub heap: Rc<WorkerHeap>,
}

impl WorkerPool {
    pub fn new() -> WorkerPool {
        let num = num_cpus::get();
        let mut workers = Vec::with_capacity(num);
        for i in 0..num {
            let (tx, rx) = channel();
            let thread_handle = spawn(move || {
                set_affinity(i);
                let mut runtime = Runtime::new().unwrap();
                {
                    tx.send(runtime.handle()).unwrap();
                }
                runtime.run().unwrap()
            });
            let runtime_handle = rx.recv().unwrap();
            workers.push(WorkerHandle {
                thread: thread_handle,
                runtime: runtime_handle,
                heap: Rc::new(WorkerHeap::new()),
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

/// set the thread in question to run on the cpu specified,
/// preferably.
fn set_affinity(cpu_num: usize) {
    let mut cpu_set = CpuSet::new();
    cpu_set.set(cpu_num).unwrap();
    // setting affinity from 0 will set it for the current
    // thread
    sched_setaffinity(Pid::from_raw(0), &cpu_set).unwrap();
}

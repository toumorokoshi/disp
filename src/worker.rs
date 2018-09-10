use futures::{
    Async,
    Future,
};
use nix::{
    sched::{sched_setaffinity, CpuSet},
    unistd::Pid,
};
use std::{
    sync::mpsc::channel,
    thread::{spawn, JoinHandle}
};
use tokio::runtime::current_thread::{Handle, Runtime};

/// Worker contain all objects required to
/// interact with a specific worker.
pub struct Worker {
    pub thread: JoinHandle<()>,
    pub runtime: Handle,
}

impl Worker {
    // spawn a worker in a new thread,
    // set affinity of the worker thread to the desired
    // cpu, and return the handle.
    pub fn spawn(cpu_num: usize) -> Worker {
        let (tx, rx) = channel();
        let thread_handle = spawn(move || {
            set_affinity(cpu_num);
            let mut runtime = Runtime::new().unwrap();
            {
                tx.send(runtime.handle()).unwrap();
            }
            runtime.spawn(WorkerController{});
            runtime.run().unwrap()
        });
        let runtime_handle = rx.recv().unwrap();
        return Worker {
            thread: thread_handle,
            runtime: runtime_handle,
        };
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

pub struct WorkerController {
}


impl Future for WorkerController {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        Ok(Async::NotReady)
    }
}

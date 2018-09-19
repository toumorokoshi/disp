use futures::{
    Async,
    future,
    Future,
    task,
};
use nix::{
    sched::{sched_setaffinity, CpuSet},
    unistd::Pid,
};
use std::{
    cell::RefCell,
    sync::mpsc::channel,
    thread::{spawn, JoinHandle}
};
use tokio::{
    runtime::current_thread::{Handle, Runtime}
};

thread_local! {
    /// if true, the worker should remain active.
    /// if false, the coroutine keeping the worker alive
    /// will die, resulting in the worker dying after
    /// all tasks are complete.
    pub static WORKER_ACTIVE: RefCell<bool> = RefCell::new(true);
}

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
            runtime.run().unwrap();
        });
        let runtime_handle = rx.recv().unwrap();
        return Worker {
            thread: thread_handle, runtime: runtime_handle,
        };
    }

    /// notify the worker to shutdown when idle
    pub fn shutdown(&self) {
        self.runtime.spawn(WorkerShutdown{});
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
        let mut result = false;
        WORKER_ACTIVE.with(|wa_ref| {
            result = *wa_ref.borrow();
        });
        return Ok(match result {
            false => {
                Async::Ready(())
            },
            true => {
                task::current().notify();
                Async::NotReady
            }
        })
    }
}


pub struct WorkerShutdown {
}


impl Future for WorkerShutdown {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        WORKER_ACTIVE.with(|wa| {
            *wa.borrow_mut() = false;
        });
        Ok(Async::Ready(()))
    }
}

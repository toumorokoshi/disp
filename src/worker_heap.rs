use std::{
    cell::RefCell,
    collections::HashMap,
};
use super::{Type, Value};

/// we declare the WorkerHeap as a thread local,
/// as it's a bit difficult to wire in the WorkerHandle
/// in the same fashion as how handlers work within Tokio
///
/// Executors within Tokio also rely on a few threadlocal
/// variables, so this looks to be an ok pattern.
thread_local! {
    pub static WORKER_HEAP: RefCell<WorkerHeap> = RefCell::new(WorkerHeap::new());
}


/// The thread heap stores references to values that are allocated in a specific thread
/// (such as reading a string from an input).
/// a ThreadHeap is preferred over alternatives such as a Heap per Fiber due to efficiency.
/// A Heap per fiber would result in increased overhead as values move from one fiber to the next
/// (e.g. when you have a processing pipeline), but could be used as a locking mechanism.
/// In this case locking is not required, as a Thread can only run one fiber at any given time,
///
/// Moving fibers from one worker to another will incur a slight cost on moving references,
/// or potentially promoting a reference to be shared across multiple workers (which would
/// from that point on require locks to write, an be more expensive). However,
/// this is expected to be a less frequent case, as fibers should spawn fibers handling the
/// same data to the same thread to take advantage of cache affinity.
///
/// TODO: figure out how to graduate strings or move them among workers.
pub struct WorkerHeap {
    // pub object_types: Vec<Type>,
    pub strings: Vec<String>
}

impl WorkerHeap {
    pub fn new() -> WorkerHeap {
        return WorkerHeap {
            strings: vec![]
        };
    }
}

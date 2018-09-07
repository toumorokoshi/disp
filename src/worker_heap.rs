use std::collections::HashMap;
use super::{Value};

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
    pub strings: HashMap<Value, String>,
}

impl WorkerHeap {
    pub fn new() -> WorkerHeap {
        return WorkerHeap {
            strings: HashMap::new()
        };
    }
}

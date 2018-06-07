use super::num_cpus;
use std::{thread, time};
use std::vec::Vec;



pub struct VM {
    workers: Vec<thread::JoinHandle<thread::Result<()>>>
}

impl VM {
    pub fn new() -> VM {
        let mut workers = vec![];
        for _ in 0..num_cpus::get() {
            workers.push(thread::spawn(|| {
                loop {
                    // TODO: add working against vm itself.
                    let one_second = time::Duration::from_millis(1000);
                    thread::sleep(one_second);
                }
            }));
        }
        return VM {
            workers: workers
        };
    }
}

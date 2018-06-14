use futures::{Async, Future};
use std::vec::{Vec};
use super::{ValueList, Op};


/// Tasks represent a single fiber on the vm.
pub struct Fiber {
    registers: ValueList,
    ops: Vec<Op>
}

impl Fiber {
    pub fn new(registers: ValueList, ops: Vec<Op>) -> Fiber {
        Fiber{
            registers: registers,
            ops: ops
        }
    }
}

impl Future for Fiber {
    // TODO: find the right value for this
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        let mut i = 0;
        while i < self.ops.len() {
            let ref op = self.ops[i];
            i += 1;
        }
        println!("hello world");
        Ok(Async::Ready(()))
    }
}

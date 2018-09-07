use std::{
    sync::Arc
};
use futures::{Async, Future};
use super::{
    ValueList,
    VMFunction,
    VMHandle
};


/// Tasks represent a single fiber on the vm.
pub struct Fiber {
    function: Arc<VMFunction>,
    vm: VMHandle,
}

impl Fiber {
    pub fn new(function: Arc<VMFunction>, vm: VMHandle) -> Fiber {
        Fiber {
            function: function,
            vm: vm,
        }
    }
}


impl Future for Fiber {
    // TODO: find the right value for this
    // Value
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        let registers = ValueList::with_capacity(self.function.registers.len());
        self.function.execute(&self.vm, registers);
        Ok(Async::Ready(()))
    }
}

use futures::{Async, Future};
use super::{Function, ValueList, VMHandle};


/// Tasks represent a single fiber on the vm.
pub struct Fiber {
    registerCount: usize,
    function: Function,
    vm: VMHandle
}

impl Fiber {
    pub fn new(registerCount: usize, function: Function, vm: VMHandle) -> Fiber {
        Fiber{
            registerCount: registerCount,
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
        let mut registers = ValueList::with_capacity(self.registerCount);
        self.function.execute(&self.vm, registers);
        Ok(Async::Ready(()))
    }
}

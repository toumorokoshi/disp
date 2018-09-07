use std::{
    rc::Rc,
    sync::Arc
};
use futures::{Async, Future};
use super::{
    WorkerHeap,
    ValueList,
    VMFunction,
    VMHandle
};


/// Tasks represent a single fiber on the vm.
pub struct Fiber {
    function: Arc<VMFunction>,
    heap: WorkerHeap,
    vm: Rc<VMHandle>,
}

impl Fiber {
    pub fn new(
        function: Arc<VMFunction>,
        heap: Rc<WorkerHeap>,
        vm: VMHandle
    ) -> Fiber {
        Fiber{
            function: function,
            heap: heap,
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
        let mut registers = ValueList::with_capacity(self.function.registers.len());
        self.function.execute(&self.vm, registers);
        Ok(Async::Ready(()))
    }
}

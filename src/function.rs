use std::sync::Arc;
use super::{Register, Type, OpList, Value, ValueList, VM};

pub type NativeFunction = fn(&mut VM, Vec<Value>) -> Value;

#[derive(Clone)]
pub enum Function {
    VM(Arc<VMFunction>),
    Native(Arc<NativeFunction>)
}

impl Function {
    pub fn execute(&self, vm: &mut VM, mut args: ValueList) -> Value {
        match self {
            &Function::VM(ref func) => func.execute(vm, args),
            &Function::Native(ref func) => {
                func(vm, args)
            }
        }
    }
}

pub struct VMFunction {
    pub registers: Vec<Type>,
    pub return_type: Type,
    pub ops: OpList
}

impl VMFunction {
    pub fn new() -> VMFunction {
        return VMFunction {
            registers: Vec::new(),
            ops: OpList::new(),
            return_type: Type::None
        };
    }

    pub fn print_ops(&self) {
        let mut i = 0;
        for ref op in &self.ops {
            println!("{}: {}", i, op);
            i += 1;
        }
    }

    pub fn execute(&self, vm: &mut VM, mut args: ValueList) -> Value {
        let target_size = args.len() + self.registers.len();
        args.resize(target_size, 0);
        // TODO: decide if this is the right pattern
        0
        // vm.execute_instructions(args, &self.ops)
    }
}

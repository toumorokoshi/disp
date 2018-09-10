use std::{
    mem,
    sync::{Arc}
};
use super::{
    Op,
    OpList,
    Type,
    WorkerHeap,
    Value,
    ValueList,
    VMHandle
};

pub type NativeFunction = fn(vm: &VMHandle, &mut WorkerHeap, &mut ValueList) -> Value;


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

    pub fn execute(&self, vm: &VMHandle, worker_heap: &mut WorkerHeap, mut args: ValueList) -> Value {
        let target_size = args.len() + self.registers.len();
        args.resize(target_size, 0);
        // TODO: once rust supports allocating
        // variable length arrays on the stack, use that
        // instead. This is heap allocated which can be significantly
        // less performant.
        let return_value = 0 as usize;
        let mut registers = args;
        let mut i = 0;
        while i < self.ops.len() {
            let ref op = self.ops[i];
            if cfg!(feature = "debug") {
                println!("DEBUG: running op {0}", op);
            }
            match op {

                &Op::Assign{source, target} => {
                    registers[target] = registers[source];
                },
                &Op::ArrayCreate{target, length_source} => {},
                &Op::ArraySet{source, target, index_source} => {},
                &Op::ArrayLoad{source, target, index_source} => {},
                &Op::BoolNot{source, target} => {
                    registers[target] = if registers[source] != 1 { 1 } else { 0 };
                },
                &Op::BoolLoad{register, constant} => {
                    registers[register] = if constant {1} else {0}
                },
                &Op::BranchTrue{condition, if_true} => {
                    if registers[condition] != 0 {
                        // -1 to allow an increment at the end of the
                        // function.
                        i = if_true - 1;
                    }
                },
                &Op::BranchFalse{condition, if_false} => {
                    if registers[condition] == 0 {
                        // -1 to allow an increment at the end of the
                        // function.
                        i = if_false - 1;
                    }
                },
                &Op::CallNative{function, ref args, target} => {
                    let mut args_to_pass = Vec::new();
                    for index in args {
                        args_to_pass.push(registers[*index]);
                    }
                    // TODO: handle nested calls
                    unsafe {
                        let func = mem::transmute::<i64, Arc<NativeFunction>>(registers[function]);
                        registers[target] = func(&vm, worker_heap, &mut args_to_pass);
                    }
                },
                &Op::IntAdd{lhs, rhs, target} => registers[target] = registers[lhs] + registers[rhs],
                &Op::IntCmp{lhs, rhs, target} => registers[target] = if registers[lhs] == registers[rhs] {1} else {0},
                &Op::IntSub{lhs, rhs, target} => registers[target] = registers[lhs] - registers[rhs],
                &Op::IntMul{lhs, rhs, target} => registers[target] = registers[lhs] * registers[rhs],
                &Op::IntDiv{lhs, rhs, target} => registers[target] = registers[lhs] / registers[rhs],
                &Op::IntLoad{register, constant} => registers[register] = constant,
                &Op::IntLessEqual{lhs, rhs, target} => registers[target] = if registers[lhs] <= registers[rhs] {1} else {0},
                &Op::IntLessThan{lhs, rhs, target} => registers[target] = if registers[lhs] < registers[rhs] {1} else {0},
                &Op::FloatAdd{lhs, rhs, target} => unsafe {
                    registers[target] = mem::transmute::<f64, i64>(
                        mem::transmute::<i64, f64>(registers[lhs]) +
                        mem::transmute::<i64, f64>(registers[rhs]),
                    );
                },
                &Op::FloatCmp{lhs, rhs, target} => unsafe {
                    registers[target] = if
                        mem::transmute::<i64, f64>(registers[lhs]) ==
                        mem::transmute::<i64, f64>(registers[rhs])
                    { 1 } else { 0 };
                },
                &Op::FloatSub{lhs, rhs, target} => unsafe {
                    registers[target] = mem::transmute::<f64, i64>(
                        mem::transmute::<i64, f64>(registers[lhs]) -
                        mem::transmute::<i64, f64>(registers[rhs]),
                    );
                },
                &Op::FloatMul{lhs, rhs, target} => unsafe {
                    registers[target] = mem::transmute::<f64, i64>(
                        mem::transmute::<i64, f64>(registers[lhs]) *
                        mem::transmute::<i64, f64>(registers[rhs]),
                    );
                },
                &Op::FloatDiv{lhs, rhs, target} => unsafe {
                    registers[target] = mem::transmute::<f64, i64>(
                        mem::transmute::<i64, f64>(registers[lhs]) /
                        mem::transmute::<i64, f64>(registers[rhs]),
                    );
                },
                &Op::FloatLoad{register, constant} => unsafe {
                    registers[register] = mem::transmute::<f64, i64>(constant)
                },
                &Op::FloatLessEqual{lhs, rhs, target} => unsafe {
                    registers[target] = if
                        mem::transmute::<i64, f64>(registers[lhs]) <=
                        mem::transmute::<i64, f64>(registers[rhs])
                    { 1 } else { 0 };
                },
                &Op::FloatLessThan{lhs, rhs, target} => unsafe {
                    registers[target] = if
                        mem::transmute::<i64, f64>(registers[lhs]) <
                        mem::transmute::<i64, f64>(registers[rhs])
                    { 1 } else { 0 };
                },
                &Op::FunctionNativeLoad{ref func_name, target} => unsafe {
                    let func = vm.heap.functions_native[func_name].clone();
                    registers[target] =
                        mem::transmute::<Arc<NativeFunction>, i64>(func);
                },
                &Op::FunctionVMLoad{func_index, target} => unsafe {
                    let func = vm.heap.functions_vm[func_index].clone();
                    registers[target] =
                        mem::transmute::<Arc<VMFunction>, i64>(func);
                },
                &Op::Goto{position} => {
                    i = position - 1;
                },
                &Op::Noop{} => {},
                // TODO: incomplete. ends up as the null pointer right now.
                &Op::StringLoad{register, ref constant} => unsafe {
                    registers[register] = mem::transmute::<Arc<String>, i64>(constant.clone());
                },
                &Op::Return{register} => { return registers[register]; },
            };
            i +=1;
        }
        0
    }
}

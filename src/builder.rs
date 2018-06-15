/// builders are designed to assist with the construction of objects
/// that are used n ghvm
use super::{VMFunction, Type, OpList, Op};
use std::collections::HashMap;

#[derive(Clone)]
pub struct BuildObject {
    pub typ: Type, // the type of object
    pub register: usize // the register where the object lives
}

pub struct FunctionBuilder {
    pub registers: Vec<Type>,
    pub ops: OpList,
    pub return_type: Option<Type>,
    pub locals: HashMap<String, BuildObject>
}

impl FunctionBuilder {
    pub fn new() -> FunctionBuilder {
        return FunctionBuilder{
            registers: Vec::new(),
            ops: OpList::new(),
            return_type: None,
            locals: HashMap::new()
        }
    }

    /// get the local var with name if it exists, or create and return the BuildObject
    /// otherwise.
    pub fn get_insert_local_var(&mut self, typ: &Type, name: &String) -> BuildObject {
        if let Some(value) = self.locals.get(name) {
            return value.clone();
        }
        let local = self.allocate_local(typ);
        self.locals.insert(name.clone(), local.clone());
        return local;
    }

    pub fn get_var(&mut self, name: &String) -> Option<BuildObject> {
        match self.locals.get(name) {
            Some(o) => Some(o.clone()),
            None => None
        }
    }

    pub fn allocate_local(&mut self, typ: &Type) -> BuildObject {
        self.registers.push(typ.clone());
        return BuildObject {
            typ: typ.clone(), register: self.registers.len() - 1
        }
    }

    pub fn add_return(&mut self, obj: &BuildObject) {
        if let Some(ref rt) = self.return_type {
            if *rt != obj.typ {
                panic!("mismatch on return type.");
            }
        }
        self.return_type = Some(obj.typ.clone());
        self.ops.push(Op::Return{register: obj.register});
    }

    pub fn load_value(&mut self, typ: &Type, value: i64) -> BuildObject {
        let local = self.allocate_local(typ);
        match typ {
            &Type::Int => {
                self.ops.push(Op::IntLoad{register: local.register, constant: value});
            },
            _ => {panic!("support todo")}
        }
        return local;
    }

    pub fn build(&mut self) -> VMFunction {
        let mut function = VMFunction::new();
        match self.return_type {
            Some(ref t) => {function.return_type = t.clone();},
            None => {function.return_type = Type::None}
        }
        function.registers = self.registers.to_owned();
        function.ops = self.ops.to_owned();
        return function;
    }
}

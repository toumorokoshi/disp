use std::sync::Arc;
use std::collections::HashMap;
use super::{NativeFunction, NativeFunctionFunc, Type, VMFunction};

/// The heap represents data and objects
/// that are shared across workers in the VM.
/// Separate from the top-level VM because
/// this must be passed around workers.
pub struct Heap {
    pub functions_native: HashMap<String, HashMap<Vec<Type>, NativeFunction>>,
    pub functions_native_funcs: Vec<Arc<NativeFunctionFunc>>,
    pub functions_vm: Vec<Arc<VMFunction>>
}


impl Heap {
    pub fn new() -> Heap {
        return Heap {
            functions_native: HashMap::new(),
            functions_native_funcs: vec![],
            functions_vm: vec![],
        };
    }

    /// add a native function to the vm heap
    pub fn add_native_func(&mut self,
        name: String, params: Vec<Type>,
        return_type: Type, func: NativeFunctionFunc
    ) {
        self.functions_native_funcs.push(Arc::new(func));
        let func_index = self.functions_native_funcs.len() - 1;
        if let Some(ref mut map) = self.functions_native.get_mut(&name) {
            map.insert(params.clone(), NativeFunction{
                return_type: return_type,
                func_index: func_index,
            });
            return;
        }
        let mut map = HashMap::new();
        map.insert(params.clone(), NativeFunction {
            return_type: return_type,
            func_index: func_index,
        });
        self.functions_native.insert(name.clone(), map);
    }

    pub fn get_native_func(&self, name: &String, arguments: Vec<Type>) -> Option<NativeFunction> {
        match self.functions_native.get(name) {
            None => None,
            Some(ref function_by_type_signature) => match function_by_type_signature.get(&arguments) {
                None => None,
                Some(native_function) => Some(native_function.clone())
            }
        }
    }
}

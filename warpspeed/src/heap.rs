use std::sync::Arc;
use std::collections::HashMap;
use super::{
    FunctionSignature,
    FunctionType,
    NativeFunction,
    VMFunction,
    Type,
};

/// The heap represents data and objects
/// that are shared across workers in the VM.
/// Separate from the top-level VM because
/// this must be passed around workers.
pub struct Heap {
    pub function_signatures: HashMap<String, HashMap<Vec<Type>, FunctionSignature>>,
    pub function_native: Vec<Arc<NativeFunction>>,
    pub function_vm: Vec<Arc<VMFunction>>,
}


impl Heap {
    pub fn new() -> Heap {
        return Heap {
            function_signatures: HashMap::new(),
            function_native: vec![],
            function_vm: vec![],
        };
    }

    /// add a native function to the vm heap
    pub fn add_native_func(&mut self,
        name: String, params: Vec<Type>,
        return_type: Type, func: NativeFunction
    ) {
        self.function_native.push(Arc::new(func));
        let func_index = self.function_native.len() - 1;
        if let Some(ref mut map) = self.function_signatures.get_mut(&name) {
            map.insert(params.clone(), FunctionSignature{
                return_type: return_type,
                function_type: FunctionType::Native,
                function_index: func_index,
            });
            return;
        }
        let mut map = HashMap::new();
        map.insert(params.clone(), FunctionSignature {
            return_type: return_type,
            function_type: FunctionType::Native,
            function_index: func_index,
        });
        self.function_signatures.insert(name.clone(), map);
    }

    /// add a native function to the vm heap
    pub fn add_vm_func(&mut self,
        name: String, params: Vec<Type>,
        return_type: Type, func: VMFunction,
    ) -> usize {
        self.function_vm.push(Arc::new(func));
        let func_index = self.function_native.len() - 1;
        if let Some(ref mut map) = self.function_signatures.get_mut(&name) {
            map.insert(params.clone(), FunctionSignature{
                return_type: return_type,
                function_type: FunctionType::Native,
                function_index: func_index,
            });
            return func_index;
        }
        let mut map = HashMap::new();
        map.insert(params.clone(), FunctionSignature {
            return_type: return_type,
            function_type: FunctionType::Native,
            function_index: func_index,
        });
        self.function_signatures.insert(name.clone(), map);
        return func_index;
    }

    pub fn get_func(&self, name: &String, arguments: Vec<Type>) -> Option<FunctionSignature> {
        match self.function_signatures.get(name) {
            None => None,
            Some(ref function_by_type_signature) => match function_by_type_signature.get(&arguments) {
                None => None,
                Some(signature) => Some(signature.clone())
            }
        }
    }
}

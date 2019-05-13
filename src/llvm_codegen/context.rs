use super::{AnnotatedFunctionMap, Compiler, Function, Scope, Type, Object, LLVMInstruction, BasicBlock, to_ptr};
use llvm_sys::{analysis::*, core::*, execution_engine::*, prelude::*, support::*, target::*, *};


pub struct Context<'a, 'b: 'a> {
    pub function_map: &'a AnnotatedFunctionMap,
    pub compiler: &'a mut Compiler<'b>,
    pub function: &'a mut Function,
    pub scope: &'a mut Scope<'b>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn new(
        function_map: &'a AnnotatedFunctionMap,
        compiler: &'a mut Compiler<'b>,
        function: &'a mut Function,
        scope: &'a mut Scope<'b>,
    ) -> Context<'a, 'b> {
        return Context {
            function_map,
            compiler,
            function,
            scope,
        };
    }

    pub fn allocate(&mut self, object_type: Type) -> Object {
        let typ = self
            .compiler
            .llvm
            .types
            .get(&object_type);
        let alloca = LLVMBuildAlloca(
            self.compiler.llvm.builder, 
            typ,
            to_ptr("alloca")
        );
        Object::new(
            alloca, object_type
        )
    }

    pub fn add_instruction(&mut self, instruction: LLVMInstruction) {
        self.function.basic_blocks[self.block].add_instruction(instruction)
    }

    pub fn allocate_without_type(&mut self) -> usize {
        self.function.allocate_object()
    }

    // add a basic block, a pointer to a section
    // of code for llvm.
    pub fn create_block(&mut self, name: String) -> usize {
        self.function.create_block(name)
    }

    pub fn current_block(&self) -> &BasicBlock {
        &self.function.basic_blocks[self.block]
    }

    pub fn get_function(&self, name: &str, arg_types: &[Type]) -> Option<String> {
        match self.scope.get_function(name, arg_types) {
            Some(function) => Some(function),
            None => match self.compiler.scope.get_function(name, arg_types) {
                Some(function) => Some(function),
                None => None,
            },
        }
    }
}
use super::{AnnotatedFunctionMap, Compiler, Function, Scope, Type, Object, LLVMInstruction, BasicBlock};

pub struct Context<'a, 'b: 'a> {
    pub function_map: &'a AnnotatedFunctionMap,
    pub compiler: &'a mut Compiler<'b>,
    pub function: &'a mut Function,
    pub scope: &'a mut Scope<'b>,
    /// this should be the current block that
    /// the builder is building against. This allows
    /// one to get back to it when switching context,
    /// for example building a child function.
    /// TODO: move current block to function
    pub block: usize,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn new(
        function_map: &'a AnnotatedFunctionMap,
        compiler: &'a mut Compiler<'b>,
        function: &'a mut Function,
        scope: &'a mut Scope<'b>,
        block: usize,
    ) -> Context<'a, 'b> {
        return Context {
            function_map,
            compiler,
            function,
            scope,
            block,
        };
    }

    pub fn allocate(&mut self, object_type: Type) -> Object {
        self.function.allocate(object_type)
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
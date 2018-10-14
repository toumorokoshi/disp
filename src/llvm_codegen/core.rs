use super::{add_native_functions, FunctionPrototype, LLVMInstruction, Scope, Type};
use std::collections::HashMap;

/// Objects are to represent values,
/// variables, and functions.
#[derive(Clone, Debug)]
pub struct Object {
    pub index: usize,
    pub object_type: Type,
    pub function_prototype: Option<FunctionPrototype>,
}

impl Object {
    pub fn new(index: usize, object_type: Type) -> Object {
        Object {
            index: index,
            object_type: object_type,
            function_prototype: None,
        }
    }

    pub fn function_prototype(function_prototype: FunctionPrototype) -> Object {
        Object {
            index: 0 as usize,
            object_type: Type::FunctionPrototype,
            function_prototype: Some(function_prototype),
        }
    }

    pub fn none() -> Object {
        Object::new(0, Type::None)
    }
}

/// Functions represent functions within disp.
#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub arg_types: Vec<Type>,
    pub return_type: Type,
    // objects store values where instructions should
    // be stored. registers are strongly typed.
    pub objects: usize,
    // a counter used simply to store indexes to basic blocks.
    pub basic_blocks: usize,
    pub instructions: Vec<LLVMInstruction>,
}

#[derive(Clone)]
pub struct NativeFunction {
    pub name: String,
    pub arg_types: Vec<Type>,
    pub return_type: Type,
}

#[derive(Clone)]
pub enum FunctionType {
    Disp(Function),
    Native(NativeFunction),
}

impl FunctionType {
    pub fn arg_types(&self) -> Vec<Type> {
        match self {
            FunctionType::Disp(f) => f.arg_types.clone(),
            FunctionType::Native(f) => f.arg_types.clone(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            FunctionType::Disp(f) => &f.name,
            FunctionType::Native(f) => &f.name,
        }
    }

    pub fn return_type(&self) -> Type {
        match self {
            FunctionType::Disp(f) => f.return_type.clone(),
            FunctionType::Native(f) => f.return_type.clone(),
        }
    }
}

impl Function {
    pub fn new(name: String, arg_types: Vec<Type>, return_type: Type) -> Function {
        Function {
            name,
            arg_types,
            return_type,
            objects: 0,
            basic_blocks: 0,
            instructions: vec![],
        }
    }

    // allocate an llvm object.
    pub fn allocate_object(&mut self) -> usize {
        let index = self.objects;
        self.objects += 1;
        index
    }
}

/// The context object contains all relevant
/// information for the Codegen to successfully build
/// llvm ode.
pub struct Context<'a, 'b: 'a> {
    pub scope: &'a mut Scope<'b>,
    pub compiler: &'a CompilerData,
    pub function: Function,
    /// this should be the current block that
    /// the builder is building against. This allows
    /// one to get back to it when switching context,
    /// for example building a child function.
    /// TODO: move current block to function
    pub block: usize,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn new(
        scope: &'a mut Scope<'b>,
        compiler: &'a CompilerData,
        function: Function,
        block: usize,
    ) -> Context<'a, 'b> {
        Context {
            scope,
            compiler,
            function,
            block,
        }
    }

    // add a basic block, a pointer to a section
    // of code for llvm.
    pub fn add_basic_block(&mut self, name: String) -> usize {
        self.function.basic_blocks += 1;
        let target = self.function.basic_blocks - 1;
        self.function
            .instructions
            .push(LLVMInstruction::AppendBasicBlock { name, target });
        target
    }

    /// add an instruction.
    pub fn add_instruction(&mut self, instruction: LLVMInstruction) {
        self.function.instructions.push(instruction);
    }

    // allocate an object of a specific size.
    pub fn allocate(&mut self, object_type: Type) -> Object {
        let index = self.function.allocate_object();
        Object::new(index, object_type.clone())
    }

    pub fn allocate_without_type(&mut self) -> usize {
        self.function.allocate_object()
    }
}

// the dispcompiler object is a global
/// that contains context for the whole
/// disp application being created.
pub struct Compiler<'a> {
    pub scope: Scope<'a>,
    pub data: CompilerData,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Compiler<'a> {
        let mut compiler = Compiler {
            scope: Scope::new(None),
            data: CompilerData::new(),
        };
        add_native_functions(&mut compiler);
        compiler
    }
}

pub struct CompilerData {
    pub functions: HashMap<String, FunctionType>,
}

impl CompilerData {
    pub fn new() -> CompilerData {
        CompilerData {
            functions: HashMap::new(),
        }
    }
}

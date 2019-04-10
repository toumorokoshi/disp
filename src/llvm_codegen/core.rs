use super::{get_builtin_expressions, BuiltinExpressions, LLVMInstruction, Scope, Type};
use std::collections::HashMap;

/// Objects are to represent values,
/// variables, and functions.
#[derive(Clone, Debug)]
pub struct Object {
    pub index: usize,
    pub object_type: Type,
}

impl Object {
    pub fn new(index: usize, object_type: Type) -> Object {
        Object {
            index: index,
            object_type: object_type,
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
    pub return_type: Option<Type>,
    // objects store values where instructions should
    // be stored. registers are strongly typed.
    pub objects: usize,
    // a counter used simply to store indexes to basic blocks.
    // pub basic_blocks: usize,
    pub basic_blocks: Vec<BasicBlock>,
}

/// BasicBlocks represent a set of statements that end with a terminator.
#[derive(Clone)]
pub struct BasicBlock {
    pub name: String,
    pub instructions: Vec<LLVMInstruction>,
    /// returns true if this BasicBlock contains a
    /// terminator. LLVM does not allow statements after a
    /// terminator.
    contains_terminator: bool,
}

impl BasicBlock {
    pub fn new(name: String) -> BasicBlock {
        BasicBlock {
            name,
            instructions: vec![],
            contains_terminator: false,
        }
    }

    /// add an LLVM instruction. this interface also validates and
    /// handles situations such as
    pub fn add_instruction(&mut self, instruction: LLVMInstruction) {
        if instruction.is_terminator() {
            self.contains_terminator = true;
        }
        self.instructions.push(instruction);
    }

    pub fn has_been_terminated(&self) -> bool {
        return self.contains_terminator;
    }
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
            FunctionType::Disp(f) => match f.return_type {
                Some(ref return_type) => return_type.clone(),
                None => Type::None,
            },
            FunctionType::Native(f) => f.return_type.clone(),
        }
    }
}

impl Function {
    pub fn new(name: String, arg_types: Vec<Type>, return_type: Option<Type>) -> Function {
        Function {
            name,
            arg_types,
            return_type,
            objects: 1,
            basic_blocks: vec![],
        }
    }

    // allocate an llvm object.
    pub fn allocate_object(&mut self) -> usize {
        let index = self.objects;
        self.objects += 1;
        index
    }

    pub fn allocate(&mut self, object_type: Type) -> Object {
        let index = self.allocate_object();
        Object::new(index, object_type)
    }

    /// add a finalized basic block to the function,
    /// returning the index by which to reference it.
    /// when constructing branches, this is a little
    /// counterintuitive because it requires constructing
    /// the blocks in reverse order from which they are executed.
    pub fn create_block(&mut self, name: String) -> usize {
        self.basic_blocks.push(BasicBlock::new(name));
        self.basic_blocks.len() - 1
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
        for expression in get_builtin_expressions().values() {
            (expression.boostrap_compiler)(&mut compiler);
        }
        // add_native_functions(&mut compiler);
        compiler
    }
}

pub struct CompilerData {
    pub functions: HashMap<String, FunctionType>,
    pub builtin_expressions: BuiltinExpressions,
}

impl CompilerData {
    pub fn new() -> CompilerData {
        CompilerData {
            functions: HashMap::new(),
            builtin_expressions: get_builtin_expressions(),
        }
    }
}

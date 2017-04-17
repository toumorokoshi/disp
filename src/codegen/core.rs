use ghvm;
use super::super::Block;

struct CodeGenerator {
    /// a vm is necessary to execute expressions.
    vm: ghvm::VM
}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        return CodeGenerator {
            vm: ghvm::VM::new()
        }
    }
}

struct Context {
    block: Block,
    function: ghvm::Function
}

impl Context {
    pub fn new() -> Context {
        return Context {
            block: Block::new(),
            function: ghvm::Function::new();
        }
    }
}

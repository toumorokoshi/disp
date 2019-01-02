use super::{Compiler, parse};

/// runs through the workflow as described
/// in compiler-design.
pub fn compile(compiler: &mut Compiler, input: &str) {
    let token = parse(input);
}


use super::{compile_module, full_parse, Compiler, DispError};
use std::{fs::File, io::Read, sync::Arc};

// load and execute a file into the vm.
pub fn exec_file(path: &str) -> Result<(), DispError> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let inp = full_parse(&input);
    let mut compiler = Compiler::new();
    let f = compile_module(&mut compiler, "main", &inp)?;
    f();
    Ok(())
}

use super::{load_file, Compiler, GenericError, GenericResult};

// for now, we can emulate a standard library
// by just loading a lib file before executing the file in question.
const LIB_FILE: &'static str = "lib.ds";

// load the standard library
// pub fn load_stdlib(vm: &mut VM) -> Result<(), DispError> {
//     exec_file(vm, LIB_FILE)
// }

pub fn load_stdlib<'a>(compiler: &mut Compiler<'a>) -> Result<(), GenericError> {
    load_file(compiler, LIB_FILE, "std")?;
    // we do not execute the stdlib file.
    Ok(())
}

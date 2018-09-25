use std::{
    fs::File,
    io::Read,
    sync::Arc,
};
use super::{
    compile,
    full_parse,
    DispError
};
use warpspeed::{VM};

// load and execute a file into the vm.
pub fn exec_file(vm: &mut VM, path: &str) -> Result<(), DispError> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let inp = full_parse(&input);
    let func = compile(vm, &inp)?;
    if cfg!(feature = "debug") {
        println!("DEBUG: ops: ");
        func.print_ops();
    }
    vm.submit(Arc::new(func), vec![]);
    Ok(())
}

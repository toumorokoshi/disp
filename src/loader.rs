use super::{parse, Builder, Compiler, GenericResult};
use std::time::Instant;
use std::{fs::File, io::Read};

// load and execute a file into the vm.
pub fn exec_file<'a>(compiler: &mut Compiler<'a>, path: &str) -> GenericResult<()> {
    load_file(compiler, path, "main")?;
    let mut builder = Builder::new(&compiler.llvm);
    builder.build(&compiler.data, &mut compiler.llvm.types);
    let f = builder.get_function("main-main")?;
    if cfg!(feature = "debug") {
        let before = Instant::now();
        f();
        println!("function duration: {}", before.elapsed().as_secs_f64());
    } else {
        f();
    }
    Ok(())
}

// load a file into the VM.
pub fn load_file<'a>(
    compiler: &mut Compiler<'a>,
    path: &str,
    module_name: &str,
) -> GenericResult<()> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let inp = parse(&input);
    // TODO: fix to compile a module
    // Ok(compile_module(compiler, module_name, &inp)?)
    Ok(())
}

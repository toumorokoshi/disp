use super::{
    annotate_types, apply_macros_to_function_map, build_functions, parse,
    parse_functions_and_macros, Builder, Compiler, GenericResult,
};
use std::time::Instant;

/// runs through the workflow as described
/// in compiler-design.
pub fn compile(compiler: &mut Compiler, input: &str) -> GenericResult<()> {
    let token = parse(input);
    if cfg!(feature = "debug") {
        println!("parsing functions...")
    }
    let (mut functions, macros) = parse_functions_and_macros(compiler, token)?;
    if cfg!(feature = "debug") {
        println!(
            "applying macros {:?} to functions: {:?}...",
            &macros, &functions
        );
    }
    apply_macros_to_function_map(&macros, &mut functions);
    if cfg!(feature = "debug") {
        println!(
            "applying annotating types for functions: {:?}...",
            &functions.keys()
        );
    }
    let annotated_functions = annotate_types(compiler, &functions)?;
    if cfg!(feature = "debug") {
        println!("building functions: {:?}...", &annotated_functions.keys());
    }
    build_functions(&mut compiler.data, &annotated_functions)?;
    let mut builder = Builder::new();
    builder.build(&compiler.data);
    let f = builder.get_function("main")?;
    if cfg!(feature = "debug") {
        let before = Instant::now();
        f();
        println!("function duration: {}", before.elapsed().as_secs_f64());
    } else {
        f();
    }
    Ok(())
}

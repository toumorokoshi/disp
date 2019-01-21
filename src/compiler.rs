use super::{
    annotate_types, apply_macros_to_function_map, build_functions, parse,
    parse_functions_and_macros, Builder, Compiler, DispResult,
};
use std::time::Instant;

/// runs through the workflow as described
/// in compiler-design.
pub fn compile(compiler: &mut Compiler, input: &str) -> DispResult<()> {
    let token = parse(input);
    let (mut functions, macros) = parse_functions_and_macros(compiler, token)?;
    apply_macros_to_function_map(&macros, &mut functions);
    let annotated_functions = annotate_types(compiler, &functions)?;
    build_functions(&mut compiler.data, &annotated_functions);
    let mut builder = Builder::new();
    builder.build(&compiler.data);
    let f = builder.get_function("main-main")?;
    if cfg!(feature = "debug") {
        let before = Instant::now();
        f();
        println!("function duration: {}", before.elapsed().as_float_secs());
    } else {
        f();
    }
    Ok(())
}
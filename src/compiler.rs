use super::{
    apply_macros_to_function_map, parse, parse_functions_and_macros, Compiler, DispResult,
};

/// runs through the workflow as described
/// in compiler-design.
pub fn compile(compiler: &mut Compiler, input: &str) -> DispResult<()> {
    let token = parse(input);
    let (functions, macros) = parse_functions_and_macros(compiler, token)?;
    apply_macros_to_function_map(&macros, &mut functions);
    Ok(())
}

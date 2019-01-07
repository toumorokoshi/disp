use super::{
    annotate_types, apply_macros_to_function_map, parse, parse_functions_and_macros, Compiler,
    DispResult,
};

/// runs through the workflow as described
/// in compiler-design.
pub fn compile(compiler: &mut Compiler, input: &str) -> DispResult<()> {
    let token = parse(input);
    let (mut functions, macros) = parse_functions_and_macros(compiler, token)?;
    apply_macros_to_function_map(&macros, &mut functions);
    let annotated_functions = annotate_types(compiler, &functions);
    Ok(())
}

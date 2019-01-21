use super::{
    AnnotatedFunction, AnnotatedFunctionMap, CodegenError, CodegenResult, Compiler, CompilerData,
    Function, FunctionType, LLVMInstruction, Object, Token,
};

pub fn build_functions(
    compiler: &mut CompilerData,
    functions: &AnnotatedFunctionMap,
) -> CodegenResult<()> {
    for (name, function_by_args) in functions {
        for (_, function) in function_by_args {
            compiler.functions.insert(
                name.to_string(),
                FunctionType::Disp(build_function(name, function)?),
            );
        }
    }
    Ok(())
}

fn build_function(name: &str, function: &AnnotatedFunction) -> CodegenResult<Function> {
    let mut function = Function::new(
        name.to_owned(),
        function.arg_types.clone(),
        Some(function.return_type.clone()),
    );
    function.instructions.push(LLVMInstruction::BuildRetVoid);
    Ok(function)
}

fn gen_token(function: &AnnotatedFunction, token: &Token) -> CodegenResult<Object> {
    Ok(match token {});
}

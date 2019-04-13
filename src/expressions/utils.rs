use super::*;

pub fn codegen_binop(
    context: &mut Context,
    args: &[Token],
    op: LLVMOpcode,
) -> CodegenResult<Object> {
    if args.len() != 2 {
        return Err(CodegenError::new(&format!(
            "binary expression should only have two arguments. found {}",
            args.len()
        )));
    };
    let lhs = gen_token(context, &args[0])?;
    let rhs = gen_token(context, &args[1])?;
    let result = context.allocate(Type::Int);
    context.add_instruction(LLVMInstruction::BuildBinOp {
        opcode: op,
        lhs: lhs.index,
        rhs: rhs.index,
        target: result.index,
    });
    Ok(result)
}

/// a convenience method to add a function to a
/// context
pub fn add_function_to_compiler(
    compiler: &mut Compiler,
    name: &str,
    return_type: Type,
    arg_types: &[Type],
    ffi_name: &str,
) {
    let mut llvm_args = Vec::with_capacity(arg_types.len());
    for arg in arg_types {
        llvm_args.push(arg.to_llvm_type());
    }
    if let None = compiler.scope.get_function(name, arg_types) {
        add_function(
            &mut compiler.data,
            &mut compiler.scope,
            name,
            FunctionType::Native(NativeFunction {
                name: ffi_name.to_owned(),
                arg_types: arg_types.to_owned(),
                return_type: return_type,
            }),
        );
    }
}

/// Adds a function to both the scope object, and the
/// compiler object. This ensures that the function can be found
/// in the desired scope, and be compiled later on by llvm.
pub fn add_function(
    compiler: &mut CompilerData,
    scope: &mut Scope,
    name: &str,
    function: FunctionType,
) {
    scope.add_function(name, &function.arg_types(), function.name().to_string());
    if cfg!(feature = "debug") {
        println!(
            "adding function {} with argument types {:?}",
            name,
            &function.arg_types()
        );
    }
    // next, we add the function to the compiler.
    compiler
        .functions
        .insert(function.name().to_string(), function);
}

pub fn call_function(
    context: &mut Context,
    func_name: &str,
    args: &[Token],
) -> CodegenResult<Object> {
    let (argument_objects, argument_types) = {
        let mut argument_objects = Vec::with_capacity(args.len());
        let mut argument_types = Vec::with_capacity(args.len());
        for arg in args {
            let result = gen_token(context, arg)?;
            argument_objects.push(result.index);
            argument_types.push(result.object_type);
        }
        (argument_objects, argument_types)
    };

    // first, check the functions declared for a valid option
    if let Some(function_by_arg_count) = context.function_map.get(func_name) {
        if let Some(function) = function_by_arg_count.get(&argument_types) {
            let object = context.allocate(function.return_type.clone());
            context.add_instruction(LLVMInstruction::BuildCall {
                name: func_name.to_owned(),
                args: argument_objects,
                target: object.index,
            });
            return Ok(object);
        }
    }
    // second, check functions in scope.
    if let Some(function_name) = context.get_function(func_name, &argument_types) {
        let return_type = context
            .compiler
            .data
            .functions
            .get(&function_name)
            .unwrap()
            .return_type();
        // TODO: refactor scope to use real function objects.
        // as a workaround we'll have a nonetype here
        let object = context.allocate(return_type);
        context.add_instruction(LLVMInstruction::BuildCall {
            name: function_name.to_owned(),
            args: argument_objects,
            target: object.index,
        });
        return Ok(object);
    }
    Err(CodegenError::new(&format!(
        "unable to find function with name {} and argument types {:?}",
        func_name, &argument_types
    )))
}

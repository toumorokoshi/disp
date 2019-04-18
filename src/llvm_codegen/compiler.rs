use super::{
    AnnotatedFunction, AnnotatedFunctionMap, BasicBlock, CodegenError, CodegenResult, Context,
    Compiler, Function, FunctionType, LLVMInstruction, Object, Scope, Token, Type,
};

pub fn build_functions(
    compiler: &mut Compiler,
    functions: &AnnotatedFunctionMap,
) -> CodegenResult<()> {
    // TODO: don't clone this. It's a waste
    // to reallocate when the full map is available.
    let function_map = functions.clone();
    for (name, function_by_args) in functions {
        for (_, function) in function_by_args {
            if cfg!(feature = "debug") {
                println!("building function {:?}", &function);
            }
            let function =
                FunctionType::Disp(build_function(&function_map, compiler, name, function)?);
            compiler.data.functions.insert(name.to_string(), function);
        }
    }
    Ok(())
}

fn build_function(
    function_map: &AnnotatedFunctionMap,
    compiler: &mut Compiler,
    name: &str,
    source_function: &AnnotatedFunction,
) -> CodegenResult<Function> {
    if cfg!(feature = "debug") {
        println!("building function {}", name);
    }
    let mut function = Function::new(
        name.to_owned(),
        source_function.arg_types.clone(),
        Some(source_function.return_type.clone()),
    );
    {
        let mut scope = Scope::new(None);
        let entry_block = function.create_block("entry".to_owned());
        let mut context = Context::new(
            function_map,
            compiler,
            &mut function,
            &mut scope,
            entry_block,
        );
        // load arguments into scope
        for i in 0..source_function.arg_types.len() {
            let param_value = context.allocate_without_type();
            context.add_instruction(LLVMInstruction::GetParam {
                arg_num: i as u32,
                target: param_value,
            });
            let param = context.allocate(source_function.arg_types[i].clone());
            let param_type = context.compiler.llvm.types.get(&source_function.arg_types[i]);
            context.add_instruction(LLVMInstruction::BuildAlloca {
                llvm_type: param_type,
                target: param.index,
            });
            context.add_instruction(LLVMInstruction::BuildStore {
                source: param_value,
                target: param.index,
            });
            context
                .scope
                .locals
                .insert(source_function.function.args[i].clone(), param.clone());
        }
        gen_token(&mut context, &source_function.function.body)?;
        if !context.current_block().has_been_terminated() {
            context.add_instruction(LLVMInstruction::BuildRetVoid {});
        }
    }
    Ok(function)
}

pub fn gen_token(context: &mut Context, token: &Token) -> CodegenResult<Object> {
    Ok(match token {
        &Token::Boolean(b) => {
            let object = context.allocate(Type::Bool);
            context.add_instruction(LLVMInstruction::ConstBool {
                value: b,
                target: object.index,
            });
            object
        }
        &Token::Map(ref m) => {
            let object = context.allocate(Type::Map(Box::new(Type::String), Box::new(Type::Int)));
            context.add_instruction(LLVMInstruction::BuildCall {
                name: String::from("create_map"),
                args: vec![],
                target: object.index,
            });
            object
        }
        &Token::None => Object::none(),
        &Token::Bytes(ref s) => {
            let object = context.allocate(Type::Bytes);
            let bytes_type = context.compiler.llvm.types.get(&Type::Bytes);
            context.add_instruction(LLVMInstruction::BuildGlobalString {
                value: *s.clone(),
                target: object.index,
            });
            context.add_instruction(LLVMInstruction::BuildAlloca {
                llvm_type: bytes_type,
                target: object.index,
            });
            context.add_instruction(LLVMInstruction::BuildAlloca {
                llvm_type: bytes_type,
                target: object.index,
            });

            object
        }
        &Token::String(ref s) => {
            let object = context.allocate(Type::String);
            context.add_instruction(LLVMInstruction::BuildGlobalString {
                value: *s.clone(),
                target: object.index,
            });
            object
        }
        &Token::Symbol(ref s) => {
            let value = match context.scope.get_local(&(*s.clone())) {
                Some(s) => {
                    let object = context.allocate(s.object_type.clone());
                    context.add_instruction(LLVMInstruction::BuildLoad {
                        source: s.index,
                        target: object.index,
                    });
                    Some(object)
                }
                None => None,
            };
            match value {
                Some(value) => value,
                None => {
                    return Err(CodegenError::new(&format!("unable to find variable {}", s)));
                }
            }
        }
        &Token::Integer(i) => {
            let object = context.allocate(Type::Int);
            context.add_instruction(LLVMInstruction::ConstInt {
                value: i,
                target: object.index,
            });
            object
        }
        &Token::List(ref tl) => gen_list(context, tl)?,
        &Token::Expression(ref tl) => gen_expr(context, tl)?,
        _ => Object::none(),
    })
}

fn gen_list(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    let mut result = Ok(Object::none());
    for t in args {
        let result_to_add = gen_token(context, t)?;
        result = Ok(result_to_add);
    }
    result
}

fn gen_expr(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    if let Some((func_token, args)) = args.split_first() {
        match func_token {
            &Token::Symbol(ref s) => compile_expr(context, s, args),
            &Token::Comment(ref c) => Ok(Object::none()),
            _ => Err(CodegenError::new(&format!(
                "first token must be a symbol for expression, found {}",
                func_token
            ))),
        }
    } else {
        Err(CodegenError::new(&format!(
            "no method found found {:?}",
            args
        )))
    }
}

fn compile_expr<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
    func_name: &'a str,
    args: &'a [Token],
) -> CodegenResult<Object> {
    let codegen_function = {
        match context.compiler.data.builtin_expressions.get(func_name) {
            Some(expression) => Some((*expression).codegen),
            None => None,
        }
    };
    if let Some(codegen) = codegen_function {
        return codegen(context, args);
    } else if let Some(function_by_arg_count) = context.function_map.get(func_name) {
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
    Ok(Object::none())
    // match func_name {
    //     symbol => match context.scope.get_macro(symbol) {
    //         None => call_function(context, symbol, args),
    //     },
    // }
}

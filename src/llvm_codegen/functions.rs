use super::{
    AnnotatedFunction, AnnotatedFunctionMap, LLVMTypeCache, to_ptr,
    LLVMCompiler, Compiler, CodegenResult, CodegenError, FunctionType, Function,
    Scope, Context, Token, Type, Object
};
use llvm_sys::{analysis::*, core::*, execution_engine::*, prelude::*, support::*, target::*, *};

pub fn populate_function_pointers(llvm: &mut LLVMCompiler, functions: &AnnotatedFunctionMap) {
    for (name, function_by_args) in functions {
        for (_, function) in function_by_args {
            let mut args = Vec::with_capacity(function.arg_types.len());
            for a in &function.arg_types {
                args.push(llvm.types.get(&a));
            }
            let function_type =
                LLVMFunctionType(
                    llvm.types.get(&function.return_type), 
                    args.as_mut_ptr(), args.len() as u32, 0);
            let llvm_function =
                LLVMAddFunction(llvm.module, to_ptr(name), function_type);
            llvm.functions.insert(name.to_string(), llvm_function);
        }
    }
}

pub fn codegen_functions(
    compiler: &mut Compiler,
    functions: &AnnotatedFunctionMap,
) -> CodegenResult<()> {
    // TODO: don't clone this. It's a waste
    // to reallocate when the full map is available.
    let function_map = functions.clone();
    for (name, function_by_args) in functions {
        for (_, function) in function_by_args {
            let function_ref = compiler.llvm.functions.get(name).unwrap();
            if cfg!(feature = "debug") {
                println!("building function {:?}", &function);
            }
            let function =
                FunctionType::Disp(codegen_function(&function_map, compiler, name, function)?);
            compiler.data.functions.insert(name.to_string(), function);
        }
    }
    Ok(())
}

fn codegen_function(
    function_map: &AnnotatedFunctionMap,
    compiler: &mut Compiler,
    name: &str,
    // this is an LLVMFunctionRef
    llvm_function: *mut LLVMValue,
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
    let main_block = LLVMAppendBasicBlockInContext(
        compiler.llvm.context,
        llvm_function,
        to_ptr("entry")
    );
    let mut scope = Scope::new(None);
        let mut context = Context::new(
            function_map,
            compiler,
            &mut function,
            &mut scope,
        );
    {
        // load arguments into scope
        for i in 0..source_function.arg_types.len() {
            let param_value = LLVMGetParam(llvm_function, i as u32);
            let param = context.allocate(
                source_function.arg_types[i].clone()
            );
            LLVMBuildStore(
                context.compiler.llvm.builder, 
                param_value, param.llvm_value
            );
            context
                .scope
                .locals
                .insert(source_function.function.args[i].clone(), param.clone());
        }
        gen_token(&mut context, &source_function.function.body)?;
        if !context.current_block().has_been_terminated() {
            LLVMBuildRetVoid(context.compiler.llvm.builder);
        }
    }
    Ok(function)
}

pub fn gen_token(context: &mut Context, token: &Token) -> CodegenResult<Object> {
    Ok(match token {
        &Token::Boolean(b) => {
            let boolean_value = LLVMConstInt(LLVMInt1Type(), if b { 1 } else { 0 } as u64, 0);
            Object::new(
                boolean_value, Type::Bool
            )
        }
        &Token::None => Object::none(),
        &Token::Bytes(ref s) => {
            let const_value = LLVMBuildGlobalString(
                context.compiler.llvm.builder,
                *s.clone(),
                to_ptr("string")
            );
            LLVMBuildGlobalString(
                value: *s.clone(),
                target: global_string_pointer,
            );
            // extract the proper subtypalex chance pove
            let global_string_pointer = context.allocate_without_type(); context.add_instruction(LLVMInstruction::BuildGlobalString {
                value: *s.clone(),
                target: global_string_pointer,
            });
            create_array(context, &Type::Byte, global_string_pointer, s.len() as i64)?
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
        // TODO: map
        _ => Object::none(),
    })
}


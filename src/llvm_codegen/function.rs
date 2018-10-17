use super::{
    gen_list, CodegenError, CodegenResult, Context, Function, FunctionType, LLVMInstruction, Scope,
    Token, Type,
};

/// Function prototypes are not-yet compiled
/// functions. These can be compiled into bytecode.
#[derive(Clone, Debug)]
pub struct FunctionPrototype {
    pub argument_symbols: Vec<String>,
    pub body: Vec<Token>,
}

/// retrieve a function object, in a few ways:
/// 1. if a compiled function already exists that matches the name
///    and function types provided, use that function
/// 2. if that is not the case, look in the existing scope to see
///    if a function prototype exists that satisfies the name and argument types.
///    if so, compile a function with that prototype, and return that.
/// If these cases are exhausted, an error is returned, since there is no
/// way such a function could be compiled with existing information.
pub fn get_or_compile_function<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
    name: &'a str,
    arg_types: &'a Vec<Type>,
) -> CodegenResult<FunctionType> {
    if let Some(llvm_name) = context.scope.get_function(name, arg_types) {
        return match context.compiler.functions.get(&llvm_name) {
            Some(func) => Ok(func.clone()),
            None => Err(CodegenError::new(&format!(
                "function with llvm_name {} not found",
                llvm_name,
            ))),
        };
    }
    if let Some(prototype) = context.scope.get_prototype(name) {
        let function = compile_function(context, prototype, name, arg_types)?;
        context
            .compiler
            .functions
            .insert(function.name.clone(), FunctionType::Disp(function.clone()));
        context
            .scope
            .add_function(name, &function.arg_types, function.name.clone());
        return Ok(FunctionType::Disp(function));
    }
    Err(CodegenError::new(&format!(
        "unable to find function with name {} accepting args {:?}",
        name, arg_types
    )))
}

pub fn compile_function<'a, 'b: 'a>(
    context: &mut Context<'a, 'b>,
    prototype: FunctionPrototype,
    name: &str,
    arg_types: &Vec<Type>,
) -> CodegenResult<Function> {
    let name_with_types = format!("{}-{:?}", name, arg_types);
    let mut args = Vec::with_capacity(arg_types.len());
    for a in arg_types {
        args.push(a.to_llvm_type());
    }
    let function = Function::new(name_with_types, arg_types.to_owned(), None);
    let mut inner_scope = Scope::new(Some(context.scope));
    let mut inner_context = Context::new(&mut inner_scope, &mut context.compiler, function, 0);
    for i in 0..prototype.argument_symbols.len() {
        let param_value = inner_context.allocate_without_type();
        inner_context.add_instruction(LLVMInstruction::GetParam {
            arg_num: i as u32,
            target: param_value,
        });
        let param = inner_context.allocate(arg_types[i].clone());
        inner_context.add_instruction(LLVMInstruction::BuildAlloca {
            llvm_type: arg_types[i].to_llvm_type(),
            target: param.index,
        });
        inner_context.add_instruction(LLVMInstruction::BuildStore {
            source: param_value,
            target: param.index,
        });
        inner_context
            .scope
            .locals
            .insert(prototype.argument_symbols[i].clone(), param.clone());
    }
    let result = gen_list(&mut inner_context, &prototype.body)?;
    if result.object_type != Type::None {
        inner_context.add_instruction(LLVMInstruction::BuildRet {
            source: result.index,
        });
        inner_context.function.return_type = Some(result.object_type);
    } else {
        inner_context.add_instruction(LLVMInstruction::BuildRetVoid);
    }
    Ok(inner_context.function)
}

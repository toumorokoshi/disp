use super::{
    gen_list, to_ptr, CodegenError, CodegenResult, Context, Function, Object, Scope, Token, Type,
};
use llvm_sys::core::*;

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
) -> CodegenResult<Function> {
    if let Some(func) = context.scope.get_function(name, arg_types) {
        return Ok(func.clone());
    }
    if let Some(prototype) = context.scope.get_prototype(name) {
        let function = compile_function(context, prototype, name, arg_types)?;
        context.scope.add_function(name, function.clone());
        return Ok(function);
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
    unsafe {
        let function_type = LLVMFunctionType(
            // we start with the void type. this will be replaced
            // once we figure out the type during codegen.
            LLVMVoidType(),
            args.as_mut_ptr(),
            args.len() as u32,
            0,
        );
        let function = LLVMAddFunction(context.module, to_ptr(&name_with_types), function_type);
        let function_block =
            LLVMAppendBasicBlockInContext(context.llvm_context, function, to_ptr("entry"));
        LLVMPositionBuilderAtEnd(context.builder, function_block);
        let mut inner_scope = Scope::new(Some(context.scope));
        let mut inner_context = Context::new(
            context.llvm_context,
            context.module,
            &mut inner_scope,
            context.builder,
            context.function,
            context.block,
        );
        for i in 0..prototype.argument_symbols.len() {
            let param = LLVMGetParam(function, i as u32);
            let result_value = LLVMBuildAlloca(
                context.builder,
                arg_types[i].to_llvm_type(),
                to_ptr(&prototype.argument_symbols[i]),
            );
            LLVMBuildStore(context.builder, param, result_value);
            inner_context.scope.locals.insert(
                prototype.argument_symbols[i].clone(),
                Object::new(result_value, arg_types[i].clone()),
            );
        }
        gen_list(&mut inner_context, &prototype.body)?;
        LLVMBuildRetVoid(context.builder);
        // TODO: set return type on function block.
        // TODO: reposition builder back to original position?
        LLVMPositionBuilderAtEnd(context.builder, context.block);
        Ok(Function {
            arg_types: arg_types.clone(),
            return_type: Type::None,
            function: function,
        })
    }
}

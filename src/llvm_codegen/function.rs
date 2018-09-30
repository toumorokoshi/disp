use super::{
    gen_list, to_ptr, CodegenError, CodegenResult, Context, Function, Object, Scope, Token, Type,
};
use llvm_sys::core::*;

/// Function prototypes are not-yet compiled
/// functions. These can be compiled into bytecode.
#[derive(Clone, Debug)]
pub struct FunctionPrototype {
    argument_symbols: Vec<String>,
    body: Vec<Token>,
}

/// retrieve a function object, in a few ways:
/// 1. if a compiled function already exists that matches the name
///    and function types provided, use that function
/// 2. if that is not the case, look in the existing scope to see
///    if a function prototype exists that satisfies the name and argument types.
///    if so, compile a function with that prototype, and return that.
/// If these cases are exhausted, an error is returned, since there is no
/// way such a function could be compiled with existing information.
pub fn get_or_compile_function<'a, 'b>(
    context: &'a mut Context<'b>,
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
        "unable to find function with name {}",
        name
    )))
}

pub fn compile_function<'a, 'b>(
    context: &'a mut Context<'b>,
    prototype: FunctionPrototype,
    name: &'a str,
    arg_types: &'a Vec<Type>,
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
            LLVMAppendBasicBlockInContext(context.compiler.llvm_context, function, to_ptr("entry"));
        LLVMPositionBuilderAtEnd(context.builder, function_block);
        let mut scope = Scope::new(Some(&context.scope));
        let mut inner_context = Context {
            builder: context.builder,
            compiler: context.compiler,
            module: context.module,
            scope: &mut scope,
        };
        for i in 0..prototype.argument_symbols.len() {
            inner_context.scope.locals.insert(
                prototype.argument_symbols[i].clone(),
                Object::new(LLVMGetParam(function, i as u32), arg_types[i].clone()),
            );
        }
        gen_list(&mut inner_context, &prototype.body)?;
        // TODO: set return type on function block.
        // TODO: reposition builder back to original position?
        Ok(Function {
            arg_types: arg_types.clone(),
            return_type: Type::None,
            function: function,
        })
    }
}

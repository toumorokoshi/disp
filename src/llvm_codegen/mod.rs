mod core;
mod error;
mod function;
pub mod native_functions;
mod productions;
mod scope;
#[cfg(test)]
mod test_native_functions;
mod types;
mod utils;
pub use self::core::{Compiler, Context, Function, Object};
use self::error::{CodegenError, CodegenResult};
use self::function::{get_or_compile_function, FunctionPrototype};
pub use self::native_functions::*;
use self::productions::{
    add_production, equals_production, fn_production, let_production, match_production,
    not_production, while_production,
};
use self::scope::Scope;
use self::types::Type;
use self::utils::{to_ptr, to_string};
use super::{DispError, Token};
use libc::c_char;
use llvm_sys::{core::*, execution_engine::*, target::*};
use std::{ffi::CStr, mem};

pub type LLVMFunction = extern "C" fn();

/// compile a module, which has two components:
/// * functions and other declarations, which will be
///   added to the VM and module context to be loaded
///   by others
/// * imperative components, which will be put into the
///   main function and available to be executed.
pub fn compile_module<'a>(
    compiler: &mut Compiler<'a>,
    // TODO: split modules up into multiple LLVM modules.
    module_name: &str,
    token: &Token,
) -> CodegenResult<()> {
    unsafe {
        let mut args = vec![];
        let function_type =
            LLVMFunctionType(LLVMVoidType(), args.as_mut_ptr(), args.len() as u32, 0);
        let main_function =
            LLVMAddFunction(compiler.llvm_module, to_ptr(module_name), function_type);
        let basic_block =
            LLVMAppendBasicBlockInContext(compiler.llvm_context, main_function, to_ptr("entry"));
        LLVMPositionBuilderAtEnd(compiler.llvm_builder, basic_block);
        add_native_functions(compiler);
        let mut context = Context::new(
            compiler.llvm_context,
            compiler.llvm_module,
            &mut compiler.scope,
            compiler.llvm_builder,
            main_function,
            basic_block,
        );
        {
            let ctx = &mut context;
            gen_token(ctx, token)?;
        }
        // LLVM functions always require a return instruction of some sort.
        LLVMBuildRetVoid(context.builder);
        // this builds the function in question for now.
        if cfg!(feature = "debug") {
            println!("llvm module:");
            LLVMDumpModule(context.module);
        }
        Ok(())
    }
}

pub fn get_function(compiler: &mut Compiler, func_name: &str) -> CodegenResult<LLVMFunction> {
    let f = unsafe {
        let mut ee = mem::uninitialized();
        LLVMLinkInMCJIT();
        LLVM_InitializeNativeTarget();
        LLVM_InitializeNativeAsmPrinter();
        let mut debug_output: *mut c_char = mem::zeroed();
        if LLVMCreateExecutionEngineForModule(&mut ee, compiler.llvm_module, &mut debug_output) != 0
        {
            println!("something went wrong with module initialization.");
            println!("{:?}", CStr::from_ptr(debug_output));
        }
        let addr = LLVMGetFunctionAddress(ee, to_ptr(func_name));
        let f: LLVMFunction = mem::transmute(addr);
        f
    };
    Ok(f)
}

fn gen_token<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
    token: &'a Token,
) -> CodegenResult<Object> {
    unsafe {
        Ok(match token {
            &Token::Boolean(b) => Object::new(
                LLVMConstInt(Type::Bool.into(), (if b { 1 } else { 0 }) as u64, 0),
                Type::Bool,
            ),
            &Token::Map(ref m) => {
                let function = LLVMGetNamedFunction(context.module, to_ptr("create_map"));
                let mut args = vec![];
                let value = LLVMBuildCall(
                    context.builder,
                    function,
                    args.as_mut_ptr(),
                    args.len() as u32,
                    to_ptr("tempmap"),
                );
                Object::new(
                    value,
                    Type::Map(Box::new(Type::String), Box::new(Type::Int)),
                )
            }
            &Token::None => Object::none(),
            &Token::String(ref s) => Object::new(
                LLVMBuildGlobalStringPtr(context.builder, to_ptr(s), to_ptr("string")),
                Type::String,
            ),
            &Token::Symbol(ref s) => {
                let value = match context.scope.locals.get(&(*s.clone())) {
                    Some(s) => {
                        let loaded_value =
                            LLVMBuildLoad(context.builder, s.value, to_ptr("loadtemp"));
                        Some(Object::new(loaded_value, s.object_type.clone()))
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
            &Token::Integer(i) => Object::new(
                LLVMConstInt(Type::Int.to_llvm_type(), i as u64, 0),
                Type::Int,
            ),
            &Token::List(ref tl) => gen_list(context, tl)?,
            &Token::Expression(ref tl) => gen_expr(context, tl)?,
            _ => Object::none(),
        })
    }
}

fn gen_expr<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
    args: &'a [Token],
) -> CodegenResult<Object> {
    if let Some((func_token, args)) = args.split_first() {
        match func_token {
            &Token::Symbol(ref s) => compile_expr(context, s, args),
            // &Token::BangSymbol(ref s) => {
            //     if **s == String::from("!macro") {
            //         build_macro(context, args)
            //     } else {
            //         run_expr(context, s, args)
            //     }
            // },
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
    match func_name {
        "+" => add_production(context, args),
        "eq" => equals_production(context, args),
        "fn" => fn_production(context, args),
        "let" => let_production(context, args),
        "match" => match_production(context, args),
        "not" => not_production(context, args),
        "while" => while_production(context, args),
        symbol => {
            let mut vm_args = Vec::with_capacity(args.len());
            let mut vm_args_types = Vec::with_capacity(args.len());
            for a in args {
                let vm_a = gen_token(context, a)?;
                vm_args.push(vm_a.value);
                vm_args_types.push(vm_a.object_type);
            }
            let function = get_or_compile_function(context, symbol, &vm_args_types)?;
            unsafe {
                let value = LLVMBuildCall(
                    context.builder,
                    function.function,
                    vm_args.as_mut_ptr(),
                    vm_args.len() as u32,
                    to_ptr("calltmp"),
                );
                Ok(Object::new(value, function.return_type))
            }
        }
    }
}

fn gen_list<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
    args: &'a [Token],
) -> CodegenResult<Object> {
    let mut result = Ok(Object::none());
    for t in args {
        let result_to_add = gen_token(context, t)?;
        result = Ok(result_to_add);
    }
    return result;
}

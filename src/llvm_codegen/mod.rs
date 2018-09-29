mod error;
mod core;
mod function;
mod types;
mod scope;
use llvm_sys:: {
    core::*
};
use super::{
    DispError,
    llvm_builder::{
        LLVMBuilder,
        to_ptr,
    },
    Token,
};
use self::core:: {
    Compiler,
    Context,
    Function,
    Object
};
use self::error::{
    CodegenError,
    CodegenResult,
};
use self::function::{
    FunctionPrototype,
    get_or_compile_function,
};
use self::scope::{
    Scope,
};
use self::types::Type;

pub type LLVMFunction = extern "C" fn();

/// compile a module, which has two components:
/// * functions and other declarations, which will be
///   added to the VM and module context to be loaded
///   by others
/// * imperative components, which will be put into the
///   main function and available to be executed.
pub fn compile_module(compiler: &mut Compiler, module_name: &str, token: &Token) -> CodegenResult<()> {
    unsafe {
       let module = LLVMModuleCreateWithNameInContext(to_ptr(module_name), compiler.llvm_context);
       let context
    }
}

pub fn compile(context: &mut Context, token: &Token) -> CodegenResult<Object> {
    Ok(gen_token(context, token)?)
}

fn gen_token(context: &mut Context, token: &Token) -> CodegenResult<Object> {
    unsafe {
        Ok(match token {
            &Token::None => Object::new(LLVMConstPointerNull(LLVMVoidType())),
            &Token::Expression(ref tl)  => gen_expr(context, tl)?,
            _ => Object::new(LLVMConstPointerNull(LLVMVoidType())),
        })
    }
}

fn gen_expr(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
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
            _ => {
                Err(CodegenError::new(&format!("first token must be a symbol for expression, found {}", func_token)))
            }
        }
    } else {
        Err(CodegenError::new("no method found"))
    }
}

fn compile_expr(context: &mut Context, func_name: &str, args: &[Token]) -> CodegenResult<Object> {
    match func_name {
        symbol => {
            let mut vm_args = Vec::with_capacity(args.len());
            let mut vm_args_types = Vec::with_capacity(args.len());
            for a in args {
                let vm_a = gen_token(context, a)?;
                vm_args.push(vm_a.value);
                vm_args_types.push(vm_a.object_type);
            }
            let function = get_or_compile_function(context, &name, &vm_arg_types)?;
            unsafe {
                LLVMBuildCall(context.builder, function,
                              vm_args.as_mut_ptr(), vm_args.len() as u32,
                              to_ptr(""));
            }
        }
    }
}


fn gen_list(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    let mut result = Ok(Object::none());
    for t in args {
        result = Ok(gen_token(context, t)?);
    }
    return result;
}

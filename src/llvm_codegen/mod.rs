mod error;
mod core;
use llvm_sys:: {
    core::*
};
use super::{
    DispError,
    LLVMBuilder,
    Token
};
use self::core:: {
    Object
};
use self::error::{
    CodegenError,
    CodegenResult,
};

pub type LLVMFunction = extern "C" fn();

pub fn compile(builder: &mut LLVMBuilder, token: &Token) -> CodegenResult<Object> {
    Ok(gen_token(builder, token)?)
}

fn gen_token(builder: &mut LLVMBuilder, token: &Token) -> CodegenResult<Object> {
    unsafe {
        Ok(match token {
            &Token::None => Object::new(LLVMConstPointerNull(LLVMVoidType())),
            &Token::Expression(ref tl)  => gen_expr(builder, tl)?,
            _ => Object::new(LLVMConstPointerNull(LLVMVoidType())),
        })
    }
}

fn gen_expr(builder: &mut LLVMBuilder, args: &[Token]) -> CodegenResult<Object> {
    if let Some((func_token, args)) = args.split_first() {
        match func_token {
            &Token::Symbol(ref s) => compile_expr(builder, s, args),
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

fn compile_expr(builder: &mut LLVMBuilder, func_name: &str, args: &[Token]) -> CodegenResult<Object> {
    match func_name {
        symbol => {
            let mut vm_args = Vec::with_capacity(args.len());
            let mut vm_args_types = Vec::with_capacity(args.len());
            for a in args {
                let vm_a = gen_token(builder, a)?;
                vm_args.push(vm_a.register as usize);
                vm_args_types.push(vm_a.typ);
            }
        }
    }
}

mod core;
mod error;
mod function;
mod macros;
pub mod native_functions;
mod productions;
mod scope;
#[cfg(test)]
mod test_native_functions;
mod types;
mod utils;
pub use self::core::{
    Compiler, CompilerData, Context, Function, FunctionType, NativeFunction, Object,
};
use self::error::{CodegenError, CodegenResult};
use self::function::{get_or_compile_function, FunctionPrototype};
use self::macros::{build_macro, expand_macro, Macro};
pub use self::native_functions::*;
use self::productions::{
    equals_production, fn_production, let_production, match_production, not_production,
    operator_production, return_production, while_production,
};
pub use self::scope::Scope;
use self::types::Type;
use self::utils::{add_function, get_function, to_ptr, to_string};
use super::{DispError, LLVMInstruction, Token};
use llvm_sys::*;

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
    let name = format!("{}-{}", module_name, "main");
    let function = {
        let mut function = Function::new(name.clone(), vec![], Some(Type::None));
        let mut context =
            Context::new(&mut compiler.scope, &mut compiler.data, function.clone(), 0);
        {
            let ctx = &mut context;
            gen_token(ctx, token)?;
        }
        // LLVM functions always require a return instruction of some sort.
        context
            .function
            .instructions
            .push(LLVMInstruction::BuildRetVoid);
        context.function
    };
    add_function(
        &mut compiler.data,
        &mut compiler.scope,
        &name,
        FunctionType::Disp(function),
    );
    Ok(())
}

fn gen_token<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
    token: &'a Token,
) -> CodegenResult<Object> {
    Ok(match token {
        &Token::Boolean(b) => {
            let object = context.allocate(Type::Bool);
            context
                .function
                .instructions
                .push(LLVMInstruction::ConstBool {
                    value: b,
                    target: object.index,
                });
            object
        }
        &Token::Map(ref m) => {
            let object = context.allocate(Type::Map(Box::new(Type::String), Box::new(Type::Int)));
            context
                .function
                .instructions
                .push(LLVMInstruction::BuildCall {
                    name: String::from("create_map"),
                    args: vec![],
                    target: object.index,
                });
            object
        }
        &Token::None => Object::none(),
        &Token::String(ref s) => {
            let object = context.allocate(Type::String);
            context
                .function
                .instructions
                .push(LLVMInstruction::BuildGlobalString {
                    value: *s.clone(),
                    target: object.index,
                });
            object
        }
        &Token::Symbol(ref s) => {
            let value = match context.scope.get_local(&(*s.clone())) {
                Some(s) => {
                    let object = context.allocate(s.object_type.clone());
                    context
                        .function
                        .instructions
                        .push(LLVMInstruction::BuildLoad {
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
            context
                .function
                .instructions
                .push(LLVMInstruction::ConstInt {
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

fn gen_expr<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
    args: &'a [Token],
) -> CodegenResult<Object> {
    if let Some((func_token, args)) = args.split_first() {
        match func_token {
            &Token::Symbol(ref s) => compile_expr(context, s, args),
            &Token::BangSymbol(ref s) => {
                if **s == String::from("macro") {
                    build_macro(context, args)
                } else {
                    Err(CodegenError::new("TBD"))
                }
            }
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
        "+" => operator_production(context, args, LLVMOpcode::LLVMAdd),
        "-" => operator_production(context, args, LLVMOpcode::LLVMSub),
        "eq" => equals_production(context, args),
        "fn" => fn_production(context, args),
        "let" => let_production(context, args),
        "match" => match_production(context, args),
        "return" => return_production(context, args),
        "not" => not_production(context, args),
        "while" => while_production(context, args),
        symbol => match context.scope.get_macro(symbol) {
            Some(disp_macro) => {
                let token = expand_macro(context, &disp_macro, args)?;
                gen_token(context, &token)
            }
            None => call_function(context, symbol, args),
        },
    }
}

fn call_function<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
    func_name: &'a str,
    args: &'a [Token],
) -> CodegenResult<Object> {
    let mut vm_args = Vec::with_capacity(args.len());
    let mut vm_args_types = Vec::with_capacity(args.len());
    for a in args {
        let vm_a = gen_token(context, a)?;
        vm_args.push(vm_a.index);
        vm_args_types.push(vm_a.object_type);
    }
    let function = get_or_compile_function(context, func_name, &vm_args_types)?;
    let object = context.allocate(function.return_type());
    context
        .function
        .instructions
        .push(LLVMInstruction::BuildCall {
            name: function.name().to_owned(),
            args: vm_args,
            target: object.index,
        });
    Ok(object)
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

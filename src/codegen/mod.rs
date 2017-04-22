mod builtins;
mod core;
mod error;

use ghvm;
use self::builtins::{
    equals_production,
    not_equals_production,
    plus_production
};
use self::core::{Context, Object, CodegenResult, Production};
use self::error::CodegenError;
use super::{ensure_symbol, Token};

// compile a token into a set of VM opcodes.
// NOTE: this can also execute code due to the compile-time
// execution support.
pub fn compile(vm: &mut ghvm::VM, token: &Token) -> Result<ghvm::Function, CodegenError> {
    let mut context = Context::new(vm);
    let result_obj = try!(gen_token(&mut context, token));
    context.builder.add_return(&result_obj.to_build_object());
    return Ok(context.builder.build());
}

fn gen_token(context: &mut Context, token: &Token) -> CodegenResult {
    match token {
        &Token::Expression(ref tl) => gen_expr(context, tl),
        &Token::List(ref tl) => Ok(add_int(context, 0)),
        &Token::Symbol(ref s) => panic!("symbol found for non-expr"),
        &Token::BangSymbol(ref s) => panic!("bang symbol found for non-expr"),
        &Token::Integer(i) => Ok(add_int(context, i)),
        &Token::Boolean(b) => Ok(add_int(context, if b {1} else {0})),
        &Token::None => Ok(add_int(context, 1))
    }
}

fn run_expr(context: &mut Context, name: &str, args: &[Token]) -> CodegenResult {
    let mut owned_args = args.to_owned();
    owned_args.insert(0, Token::Symbol(Box::new(String::from(name))));
    let ref mut vm = context.vm;
    let func = try!(compile(vm, &Token::Expression(owned_args)));
    let result = vm.execute_function(&func);
    let value = context.builder.load_value(&func.return_type, result);
    Ok(Object::from_build_object(value))
}

fn compile_expr(context: &mut Context, func_name: &str, args: &[Token]) -> CodegenResult {
    let func: Production = match func_name {
        "+" => plus_production as Production,
        "=" => equals_production as Production,
        "!=" => not_equals_production as Production,
        _ => {return Err(String::from("no function found."))}
    };
    return func(context, args);
}

fn gen_expr(context: &mut Context, args: &[Token]) -> CodegenResult {
    if let Some((func_token, args)) = args.split_first() {
        return match func_token {
            &Token::Symbol(ref s) => compile_expr(context, s, args),
            &Token::BangSymbol(ref s) => run_expr(context, s, args),
            _ => Err(String::from("first token must be a symbol for expression"))
        };
    }
    Err(String::from("no method found"))
}

fn add_int(context: &mut Context, value: i64) -> Object {
    let obj = context.builder.allocate_local(&ghvm::Type::Int);
    context.builder.ops.push(ghvm::Op::IntLoad{register: obj.register, constant: value});
    Object::from_build_object(obj)
}

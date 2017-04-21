mod builtins;
mod core;
mod error;

use ghvm;
use self::builtins::plus_production;
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

fn gen_expr(context: &mut Context, expr: &Vec<Token>) -> CodegenResult {
    let mut func: Option<Production> = None;
    if let Some((func_token, args)) = expr.split_first() {
        let name = ensure_symbol(func_token);
        match name {
            "+" => { func = Some(plus_production as Production); },
            _ => {}
        }
        match func {
            Some(f) => {return f(context, args);},
            None => {}
        }
    }
    return Err(String::from("no method found"));
}

fn add_int(context: &mut Context, value: i64) -> Object {
    let obj = context.builder.allocate_local(ghvm::Type::Int);
    context.builder.ops.push(ghvm::Op::IntLoad{register: obj.register, constant: value});
    return Object::from_build_object(obj);
}

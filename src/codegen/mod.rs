mod builtins;
mod core;
mod error;

use warpspeed::{Op, Type, VM, VMFunction};
use self::builtins::{
    equals_production,
    function_production,
    if_production,
    not_equals_production,
    const_production,
    mut_production,
    plus_production,
    minus_production,
    while_production,
    match_production,
};
use self::core::{Context, Object, CodegenResult, Production};
use self::error::CodegenError;
use super::{Token};

// compile a token into a set of VM opcodes.
// NOTE: this can also execute code due to the compile-time
// execution support.
pub fn compile(vm: &mut VM, token: &Token) -> Result<VMFunction, CodegenError> {
    let mut context = Context::new(vm);
    let result_obj = try!(gen_token(&mut context, token));
    context.builder.add_return(&result_obj.to_build_object());
    return Ok(context.builder.build());
}

fn gen_token(context: &mut Context, token: &Token) -> CodegenResult {
    match token {
        &Token::Expression(ref tl) => gen_expr(context, tl),
        &Token::List(ref tl) => gen_list(context, tl),
        // TODO: represent dictionaries in the VM
        &Token::Dict(_) => Ok(add_int(context, 0)),
        &Token::Symbol(ref s) => evaluate_symbol(context, s),
        &Token::BangSymbol(ref s) => Err(format!("bang symbol {} found for non-expr", s)),
        &Token::Integer(i) => Ok(add_int(context, i)),
        &Token::Boolean(b) => Ok(Object::from_build_object({
            let obj = context.builder.allocate_local(&Type::Bool);
            context.builder.ops.push(Op::BoolLoad{register: obj.register, constant: b});
            obj
        })),
        &Token::None => Ok(Object{typ: Type::None, register: 0})
    }
}

fn run_expr(context: &mut Context, name: &str, args: &[Token]) -> CodegenResult {
    let mut owned_args = args.to_owned();
    owned_args.insert(0, Token::Symbol(Box::new(String::from(name))));
    let func = {
        let ref mut vm = context.vm;
        try!(compile(vm, &Token::Expression(owned_args)))
    };
    let result = func.execute(&context.vm.handle(), vec![]);
    let value = context.builder.load_value(&func.return_type, result);
    Ok(Object::from_build_object(value))
}

fn compile_expr(context: &mut Context, func_name: &str, args: &[Token]) -> CodegenResult {
    let func: Production = match func_name {
        "=" => equals_production as Production,
        "if" => if_production as Production,
        "!=" => not_equals_production as Production,
        "+" => plus_production as Production,
        "-" => minus_production as Production,
        "const" => const_production as Production,
        "mut" => mut_production as Production,
        "while" => while_production as Production,
        "match" => match_production as Production,
        "fn" => function_production as Production,
        "print" => {
            let result = context.builder.allocate_local(&Type::None);
            let function = context.builder.allocate_local(&Type::FunctionNative);
            let print_arg = try!(gen_token(context, &args[0]));
            let args = vec![print_arg.register];
            context.builder.ops.push(Op::FunctionNativeLoad{
                func_name: String::from("print"),
                target: function.register,
            });
            context.builder.ops.push(Op::CallNative{
                function: function.register,
                args: args,
                target: result.register,
            });
            return Ok(Object{typ: Type::None, register: 0});
        },
        _ => {return Err(String::from("no function found."))}
    };
    return func(context, args);
}

fn gen_list(context: &mut Context, args: &[Token]) -> CodegenResult {
    let mut result = Err(String::from("0 size list"));
    for t in args {
        result = gen_token(context, t);
    }
    return result;
}

fn gen_expr(context: &mut Context, args: &[Token]) -> CodegenResult {
    if let Some((func_token, args)) = args.split_first() {
        return match func_token {
            &Token::Symbol(ref s) => compile_expr(context, s, args),
            &Token::BangSymbol(ref s) => run_expr(context, s, args),
            _ => {
                println!("{}", func_token);
                Err(String::from("first token must be a symbol for expression"))
            }
        };
    }
    Err(String::from("no method found"))
}

fn evaluate_symbol(context: &mut Context, symbol: &String) -> CodegenResult {
    match context.builder.get_var(symbol) {
        Some(obj) => Ok(Object::from_build_object(obj)),
        None => Err(format!("unable to find symbol {}", symbol))
    }
}

fn add_int(context: &mut Context, value: i64) -> Object {
    let obj = context.builder.allocate_local(&Type::Int);
    context.builder.ops.push(Op::IntLoad{register: obj.register, constant: value});
    Object::from_build_object(obj)
}

mod builtins;
mod core;
mod function;
mod error;

use warpspeed::{
    Op, Type, WORKER_HEAP, VM,
    FunctionType,
    VMFunction,
};
use self::builtins::{
    equals_production,
    if_production,
    not_equals_production,
    const_production,
    mut_production,
    plus_production,
    minus_production,
    while_production,
    match_production,
};
use self::function::{
    call_function,
    function_production,
    FunctionPrototype,
};
use self::core::{
    Context, Object, CodegenResult, Production,
    function_prototype
};
use self::error::CodegenError;
use super::{Token};

// compile a token into a set of VM opcodes.
// NOTE: this can also execute code due to the compile-time
// execution support.
pub fn compile(vm: &mut VM, token: &Token) -> Result<VMFunction, CodegenError> {
    let mut context = Context::new(vm);
    let result_object = gen_token(&mut context, token)?;
    context.builder.add_return(&result_object.to_build_object());
    Ok(context.builder.build())
}


fn gen_token(context: &mut Context, token: &Token) -> CodegenResult {
    match token {
        &Token::Expression(ref tl) => gen_expr(context, tl),
        &Token::List(ref tl) => gen_list(context, tl),
        // TODO: represent dictionaries in the VM
        &Token::Map(_) => Ok(create_map(context)),
        &Token::Symbol(ref s) => evaluate_symbol(context, s),
        &Token::BangSymbol(ref s) => Err(format!("bang symbol {} found for non-expr", s)),
        &Token::Integer(i) => Ok(add_int(context, i)),
        &Token::Boolean(b) => Ok(Object::from_build_object({
            let obj = context.builder.allocate_local(&Type::Bool);
            context.builder.ops.push(Op::BoolLoad{register: obj.register, constant: b});
            obj
        })),
        &Token::None => Ok(Object::none()),
        &Token::String(ref s) => {
            let obj = context.builder.allocate_local(&Type::String);
            context.builder.ops.push(Op::StringLoad{
                register: obj.register, constant: (**s).clone()
            });
            Ok(Object::from_build_object(obj))
        }
    }
}

fn run_expr(context: &mut Context, name: &str, args: &[Token]) -> CodegenResult {
    if cfg!(feature = "debug") {
        println!("DEBUG running expression: {}", name);
    }
    let mut owned_args = args.to_owned();
    owned_args.insert(0, Token::Symbol(Box::new(String::from(name))));
    let func = {
        let ref mut vm = context.vm;
        match compile(vm, &Token::Expression(owned_args.clone())) {
            Ok(func) => func,
            Err(details) => {
                return Err(format!("unable to execute expression on compile time: {:?}, {:?}", owned_args, details));
            }
        }
    };
    let result = WORKER_HEAP.with(|worker_heap| {
        func.execute(&context.vm.handle(), &mut worker_heap.borrow_mut(), &mut vec![])
    });
    let value = context.builder.load_value(&func.return_type, result);
    return Ok(Object::from_build_object(value));
}

fn compile_expr(context: &mut Context, func_name: &str, args: &[Token]) -> CodegenResult {
    let func: Production = match func_name {
        "eq" => equals_production as Production,
        "if" => if_production as Production,
        "neq" => not_equals_production as Production,
        "+" => plus_production as Production,
        "-" => minus_production as Production,
        "const" => const_production as Production,
        "mut" => mut_production as Production,
        "while" => while_production as Production,
        "match" => match_production as Production,
        "fn" => function_production as Production,
        symbol => {
            let mut vm_args = Vec::with_capacity(args.len());
            let mut vm_args_types = Vec::with_capacity(args.len());
            for a in args {
                let vm_a = gen_token(context, a)?;
                vm_args.push(vm_a.register);
                vm_args_types.push(vm_a.typ);
            }

            if let Some(func) = context.vm.heap.get_func(&String::from(symbol), vm_args_types.clone()) {
                let function_register = context.builder.allocate_local(
                    &Type::Function(Box::new(vm_args_types.clone()), Box::new(func.return_type.clone()))
                );
                let result = context.builder.allocate_local(&func.return_type);
                match func.function_type {
                    FunctionType::Native => {
                        context.builder.ops.push(Op::FunctionNativeLoad{
                            func_index: func.function_index,
                            target: function_register.register,
                        });
                        context.builder.ops.push(Op::CallNative{
                            function: function_register.register,
                            args: vm_args,
                            target: result.register,
                        });
                    },
                    // TODO: do a check first for local variable function assignment.
                    // right now this will only resolve if the heap has a function of the
                    // specific name, but it should allow for arbitrary re-assignment of function
                    // names.
                    FunctionType::VM => {
                        context.builder.ops.push(Op::FunctionVMCall{
                            function: function_register.register,
                            args: vm_args,
                            target: result.register,
                        });
                    }
                }
                return Ok(Object::from_build_object(result));
            }
            // call_function(context, &String::from(symbol), &args)
        }
    };
    return func(context, args);
}

fn gen_list(context: &mut Context, args: &[Token]) -> CodegenResult {
    let mut result = Ok(Object::new(Type::None, 0));
    for t in args {
        result = Ok(gen_token(context, t)?);
    }
    return result;
}

fn gen_expr(context: &mut Context, args: &[Token]) -> CodegenResult {
    if let Some((func_token, args)) = args.split_first() {
        match func_token {
            &Token::Symbol(ref s) => compile_expr(context, s, args),
            &Token::BangSymbol(ref s) => run_expr(context, s, args),
            _ => {
                Err(format!("first token must be a symbol for expression, found {}", func_token))
            }
        }
    } else {
        Err(String::from("no method found"))
    }
}

fn evaluate_symbol(context: &mut Context, symbol: &String) -> CodegenResult {
    match context.builder.get_var(symbol) {
        Some(obj) => Ok(Object::from_build_object(obj)),
        None => Err(format!("unable to find symbol {}", symbol)),
    }
}

fn add_int(context: &mut Context, value: i64) -> Object {
    let obj = context.builder.allocate_local(&Type::Int);
    context.builder.ops.push(Op::IntLoad{register: obj.register, constant: value});
    Object::from_build_object(obj)
}

fn create_map(context: &mut Context) -> Object {
    let obj = context.builder.allocate_local(&Type::Map(
            Box::new(Type::String), Box::new(Type::Bool)
    ));
    context.builder.ops.push(Op::MapCreate{target: obj.register});
    Object::from_build_object(obj)
}

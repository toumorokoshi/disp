use super::{
    super::GenericResult, AnnotatedFunction, AnnotatedFunctionMap, CodegenError, CodegenResult,
    Compiler, CompilerData, Function, FunctionType, LLVMInstruction, Object, Scope, Token, Type,
};

pub struct Context<'a, 'b: 'a> {
    pub compiler: &'a CompilerData,
    pub function: &'a mut Function,
    pub scope: &'a mut Scope<'b>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn new(
        compiler: &'a mut CompilerData,
        function: &'a mut Function,
        scope: &'a mut Scope<'b>,
    ) -> Context<'a, 'b> {
        return Context {
            compiler,
            function,
            scope,
        };
    }

    pub fn allocate(&mut self, object_type: Type) -> Object {
        self.function.allocate(object_type)
    }

    pub fn add_instruction(&mut self, instruction: LLVMInstruction) {
        self.function.instructions.push(instruction);
    }
}

pub fn build_functions(
    compiler: &mut CompilerData,
    functions: &AnnotatedFunctionMap,
) -> CodegenResult<()> {
    for (name, function_by_args) in functions {
        for (_, function) in function_by_args {
            if cfg!(feature = "debug") {
                println!("building function {:?}", &function);
            }
            let function = FunctionType::Disp(build_function(compiler, name, function)?);
            compiler.functions.insert(name.to_string(), function);
        }
    }
    Ok(())
}

fn build_function(
    compiler: &mut CompilerData,
    name: &str,
    source_function: &AnnotatedFunction,
) -> CodegenResult<Function> {
    if cfg!(feature = "debug") {
        println!("building function {}", name);
    }
    let mut function = Function::new(
        name.to_owned(),
        source_function.arg_types.clone(),
        Some(source_function.return_type.clone()),
    );
    {
        let mut scope = Scope::new(None);
        let mut context = Context::new(compiler, &mut function, &mut scope);
        gen_token(&mut context, &source_function.function.body)?;
    }
    function.instructions.push(LLVMInstruction::BuildRetVoid);
    Ok(function)
}

pub fn gen_token(context: &mut Context, token: &Token) -> CodegenResult<Object> {
    Ok(match token {
        &Token::Boolean(b) => {
            let object = context.allocate(Type::Bool);
            context.add_instruction(LLVMInstruction::ConstBool {
                value: b,
                target: object.index,
            });
            object
        }
        &Token::Map(ref m) => {
            let object = context.allocate(Type::Map(Box::new(Type::String), Box::new(Type::Int)));
            context.add_instruction(LLVMInstruction::BuildCall {
                name: String::from("create_map"),
                args: vec![],
                target: object.index,
            });
            object
        }
        &Token::None => Object::none(),
        &Token::String(ref s) => {
            let object = context.allocate(Type::String);
            context.add_instruction(LLVMInstruction::BuildGlobalString {
                value: *s.clone(),
                target: object.index,
            });
            object
        }
        &Token::Symbol(ref s) => {
            let value = match context.scope.get_local(&(*s.clone())) {
                Some(s) => {
                    let object = context.allocate(s.object_type.clone());
                    context.add_instruction(LLVMInstruction::BuildLoad {
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
            context.add_instruction(LLVMInstruction::ConstInt {
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

fn gen_list(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    let mut result = Ok(Object::none());
    for t in args {
        let result_to_add = gen_token(context, t)?;
        result = Ok(result_to_add);
    }
    result
}

fn gen_expr(context: &mut Context, args: &[Token]) -> CodegenResult<Object> {
    if let Some((func_token, args)) = args.split_first() {
        match func_token {
            &Token::Symbol(ref s) => compile_expr(context, s, args),
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
    if let Some(expression) = context.compiler.builtin_expressions.get(func_name) {
        return ((*expression).codegen)(context, args);
    }
    Ok(Object::none())
    // match func_name {
    //     symbol => match context.scope.get_macro(symbol) {
    //         None => call_function(context, symbol, args),
    //     },
    // }
}

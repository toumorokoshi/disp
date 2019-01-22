use super::{
    AnnotatedFunction, AnnotatedFunctionMap, CodegenError, CodegenResult, Compiler, CompilerData,
    Function, FunctionType, LLVMInstruction, Object, Scope, Token, Type,
};

struct Context<'a, 'b: 'a> {
    function: &'a mut Function,
    scope: &'a mut Scope<'b>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn new(function: &'a mut Function, scope: &'a mut Scope<'b>) -> Context<'a, 'b> {
        return Context { function, scope };
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
            compiler.functions.insert(
                name.to_string(),
                FunctionType::Disp(build_function(name, function)?),
            );
        }
    }
    Ok(())
}

fn build_function(name: &str, source_function: &AnnotatedFunction) -> CodegenResult<Function> {
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
        let mut context = Context::new(&mut function, &mut scope);
        gen_token(&mut context, &source_function.function.body)?;
    }
    function.instructions.push(LLVMInstruction::BuildRetVoid);
    Ok(function)
}

fn gen_token(context: &mut Context, token: &Token) -> CodegenResult<Object> {
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
    Ok(Object::none())
}

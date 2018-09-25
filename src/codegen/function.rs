use std::sync::Arc;
use super::{
    compile,
    CodegenResult,
    Context,
    function_prototype,
    gen_token,
    Object,
    Op,
    Token,
    Type,
    VMFunction,
};

/// represents the function prototype, that can then be
/// used to construct functions
#[derive(Clone)]
pub struct FunctionPrototype {
    pub arguments: Vec<String>,
    pub body: Vec<Token>,
}

impl FunctionPrototype {
    /// build a function with the specified types as arguments
    fn build(&self, context: &mut Context, passed_types: Vec<Type>) -> Result<VMFunction, String> {
        if passed_types.len() != self.arguments.len() {
            return Err(String::from("number of arguments passed to function is mismatched"));
        }
        let mut inner_context = Context::new(&mut context.vm);
        // insert arguments as values.
        // declare all args as passed values.
        for i in 0..passed_types.len() {
            inner_context.builder.get_insert_local_var(&passed_types[i], &self.arguments[i]);
        }
        // then, we generate the body.
        let result_object = gen_token(&mut inner_context, &Token::List(self.body.clone()))?;
        context.builder.add_return(&result_object.to_build_object());
        Ok(context.builder.build())
    }
}


/// A function production is run when a function definition is encountered.
/// This does nothing at this point, aside from declare and store a
/// function in the locals map.
/// The function itself is compiled and added to the vm on execution, enabling
/// Type inference.
pub fn function_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    let arguments = match args[0] {
        Token::List(ref argument_names) => {
            let mut arguments_builder = Vec::with_capacity(argument_names.len());
            for token in argument_names {
                match token {
                    Token::Symbol(ref name) => {
                        arguments_builder.push(*name.clone());
                    },
                    t => { return Err(format!("all arguments in an argument declaration should be a symbol. found {}", t));}
                }
            }
            arguments_builder
        },
        _ => {return Err(String::from("arguments declarations for function should be a list"));}
    };
    let body = match args[1] {
        Token::List(ref _body) => _body.clone(),
        _ => {return Err(String::from("body of argument should be a list of tokens"));}
    };
    context.block.function_prototypes.push(FunctionPrototype{
        arguments: arguments,
        body: body
    });
     // TODO: this should be a function prototype, but we'll use
     // an empty function signature for now...
     Ok(Object{
         typ: function_prototype(),
         register: 0,
         function_index: Some(context.block.function_prototypes.len() - 1)
     })
}

/// call a function
pub fn call_function(context: &mut Context, name: &String, args: &[Token]) -> CodegenResult {
    match context.block.get_local(name) {
        None => Err(format!("no such function declaration {} found", name)),
        Some(prototype_index) => {
            let prototype_function = context.block.function_prototypes[prototype_index].clone();

            let mut vm_args = Vec::with_capacity(args.len());
            let mut vm_args_types = Vec::with_capacity(args.len());

            // first, we construct argument values + types
            for a in args {
                let vm_a = gen_token(context, a)?;
                vm_args.push(vm_a.register);
                vm_args_types.push(vm_a.typ);
            };

            // next, we construct the inner function.
            let function = prototype_function.build(context, vm_args_types.clone())?;
            let return_type = function.return_type.clone();
            // now that it's constructed, we add it to the VM.
            let func_index = match Arc::get_mut(&mut context.vm.heap) {
                Some(heap) => {
                    heap.add_vm_func(name.clone(), vm_args_types.clone(), return_type.clone(), function)
                },
                None => { panic!("unable to warmup vm");}
            };
            let function_register = context.builder.allocate_local(
                &Type::Function(Box::new(vm_args_types.clone()), Box::new(return_type.clone()))
            );
            context.builder.ops.push(Op::FunctionVMLoad{
                func_index: func_index,
                target: function_register.register,
            });
            let result = context.builder.allocate_local(&return_type);
            context.builder.ops.push(Op::FunctionVMCall{
                function: function_register.register,
                args: vm_args,
                target: result.register,
            });
            Ok(Object::from_build_object(result))
        }
    }
}

pub fn _function_production(context: &mut Context, args: &[Token]) -> CodegenResult {
    // the first argument is a list of variables, so we pull those.
    // TODO: parse into VMFunction declaration.
    let _variables = try!(gen_token(context, &args[0]));
    let function = compile(&mut context.vm, &args[1]).unwrap();
    // add the function to the VM, so it can be referenced in bytecode.
    match Arc::get_mut(&mut context.vm.heap) {
        None => Err(String::from("unable to get add a method to the vm (unable to get a heap handle)")),
        Some(heap) => {
            heap.function_vm.push(Arc::new(function));
            let function_index = heap.function_vm.len() - 1;
            let function_register = context.builder.allocate_local(
                &Type::Function(Box::new(vec![]), Box::new(Type::None))
            );
            context.builder.ops.push(Op::FunctionVMLoad{
                func_index: function_index,
                target: function_register.register,
            });
            Ok(Object::from_build_object(function_register))
        }
    }
}



// call a VM function. This method
// also includes the logic to compile new
// vm level functions in the case that the
// function in question has not been called with
// the specific type signature.
// pub fn call_vm_function(context: &mut Context, name: &string, args: &[Token]) -> CodegenResult {
//     match block
// }

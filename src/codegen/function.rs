use std::sync::Arc;
use super::{
    compile,
    CodegenResult,
    Context,
    gen_token,
    Object,
    Op,
    Token,
    Type,
};


/// A function production is run when a function definition is encountered.
/// This does nothing at this point, aside from declare and store a
/// function in the locals map.
/// The function itself is compiled and added to the vm on execution, enabling
/// Type inference.
// pub fn function_production(context: &mut Context, args: &[Token]) -> CodegenResult {
//     context.block.insert()
// }

pub fn function_production(context: &mut Context, args: &[Token]) -> CodegenResult {
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
            let function_register = context.builder.allocate_local(&Type::FunctionVM);
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

/// Native functions that are available as functions within disp.
/// Functions within this module must be publicly exported in the main.rs
/// file, or else LLVM will be unable to discover the externs.
use super::{to_ptr, Context, Function, Type};
use llvm_sys::core::*;

// add native functions to a module context, to ensure
// these builtins are available.
pub fn add_native_functions(context: &mut Context) {
    unsafe {
        let mut args = vec![LLVMInt64Type()];
        let function = LLVMAddFunction(
            context.module,
            to_ptr("print"),
            LLVMFunctionType(LLVMInt64Type(), args.as_mut_ptr(), args.len() as u32, 0),
        );
        context.scope.add_function(
            "print",
            Function {
                arg_types: vec![Type::Int],
                return_type: Type::None,
                function: function,
            },
        );

        let function = LLVMAddFunction(
            context.module,
            to_ptr("println"),
            LLVMFunctionType(LLVMInt64Type(), args.as_mut_ptr(), args.len() as u32, 0),
        );
        context.scope.add_function(
            "print",
            Function {
                arg_types: vec![Type::Int],
                return_type: Type::None,
                function: function,
            },
        );
    }
}

// no_mangle is required, to ensure that
// it resolves the name that's specified by the method
// signature.
#[no_mangle]
pub extern "C" fn print(value: i64) {
    print!("{}", value);
}

#[no_mangle]
pub extern "C" fn println(value: i64) {
    println!("{}", value);
}

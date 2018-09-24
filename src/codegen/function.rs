use super::{Context, Token};


/// call a VM function. This method
/// also includes the logic to compile new
/// vm level functions in the case that the
/// function in question has not been called with
/// the specific type signature.
pub fn call_vm_function(context: &mut Context, name: &string, args: &[Token]) -> CodegenResult {
    match block
}

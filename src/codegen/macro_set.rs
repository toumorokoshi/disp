use super::{
    CodegenResult,
    Context,
    Token,
    Object
};

pub struct MacroSet {
}

pub fn build_macro(_context: &mut Context, _args: &[Token]) -> CodegenResult {
    Ok(Object::none())
}

/// constraints represent
/// the various type relationships
/// that must be preserved.
use std::rc::Rc;

pub enum Constraint {
    Equals(Rc<Type>, Rc<Type>),
}

/// types can be a type variable
/// or a type constant
pub enum Type {}

pub struct Type {}

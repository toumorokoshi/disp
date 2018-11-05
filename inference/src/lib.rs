/// constraints represent
/// the various type relationships
/// that must be preserved.
use std::rc::Rc;
mod heeren;

pub fn infer_type() {}

/// Type Inference algorithms work by defining expressions, which effectively
/// define constraints on what the types themselves can be.
/// This enum defines the available constraints,
/// using the naming standardized by Hindley-Milner.
pub enum Expression {
    // /// Var declares a new variable that a type variable may resolve to.
// Var(Type),
// /// App declares the result
// App(Box<Expression>, Box<Expression>),
// /// Abs declares that for some input Type, there
// /// exists a transformation of that type to the right-hand-side
// /// expression.
// Abs(Type, Expression),
// Equals(Rc<Type>, Rc<Type>),
// IsGeneric(Rc<Type>, Rc<Type>),
}
// /// The TypeEnvironment maps type references to their
// /// real type values
// pub struct TypeEnvironment {
//     mapping: HashMap<Type, PolyType>,
// }

/// TODO: make this more generic, potentially allowing
/// consumption of types.
pub type Type = String;

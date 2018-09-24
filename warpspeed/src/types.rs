// warpspeed types can be categorized between two types: primitives and structs
/// structs are constructed from primitives. Primitives are:
/// * integer
/// * float
/// * bool
/// number types will be split into sizings in the future.
/// It would be nice if structs were similary to how they are in c,
/// where fields that fit inside a word can be compacted.
use std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Array(Box<Type>),
    Bool,
    Float,
    FunctionNative,
    FunctionVM,
    Map(Box<Type>, Box<Type>),
    None,
    Int,
    String,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Type::Array(ref t) => write!(f, "Array<{0}>", t),
            &Type::Bool => write!(f, "Bool"),
            &Type::Float => write!(f, "Float"),
            &Type::FunctionNative => write!(f, "FunctionNative"),
            &Type::FunctionVM => write!(f, "FunctionVM"),
            &Type::Int => write!(f, "Int"),
            &Type::Map(ref key, ref value) => write!(f, "Map<{0}, {1}>", key, value),
            &Type::None => write!(f, "None"),
            &Type::String => write!(f, "String"),
        }
    }
}

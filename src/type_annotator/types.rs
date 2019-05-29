use inference::Resolved;
use super::{Type, DispError, DispResult};

/// Types used for the type
/// checking and annotation phase
#[derive(Clone, Debug, PartialEq)]
pub enum TypecheckType {
    Array,
    Bool,
    Byte,
    Int,
    None,
    String
}

pub fn to_type(resolved_type: &Resolved<TypecheckType>) ->  DispResult<Type> {
    match resolved_type {
        Resolved::Literal(ref literal) => match literal {
            &TypecheckType::Bool => Ok(Type::Bool),
            &TypecheckType::Byte => Ok(Type::Byte),
            &TypecheckType::Int => Ok(Type::Int),
            &TypecheckType::None => Ok(Type::None),
            &TypecheckType::String => Ok(Type::String),
            _ => Err(DispError::new(&format!("invalid resolved type {:?}", resolved_type)))
        },
        Resolved::Generic(ref generic, ref subtypes) => match generic {
            &TypecheckType::Array => {
                let subtype = to_type(&subtypes[0])?;
                Ok(Type::Array(Box::new(subtype)))
            },
            _ => Err(DispError::new(&format!("invalid resolved type {:?}", resolved_type)))
        }
    }
}
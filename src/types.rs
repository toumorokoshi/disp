use std::collections::HashSet;
use inference::UnificationTypes; 

/// The type enum is used to define types for Disp's
/// type checker.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Any,
    Array(Box<Type>),
    Bool,
    Byte,
    FunctionPrototype,
    Int,
    None,
    String,
    Map(Box<Type>, Box<Type>),
}

impl UnificationTypes for Type {
    fn unify(left: &Type, right: &Type) -> Result<Type, String> {
        match left {
            // Any is the most generic: return the right type if so.
            Type::Any => Ok(right.clone()),
            Type::Array(ref typ) => {
                if let Type::Array(ref other_type) = right {
                    Ok(Type::Array(Box::new(Self::unify(
                        &(*typ),
                        &(*other_type),
                    )?)))
                } else {
                    Err(String::from("unable to unify array types"))
                }
            }
            normal_type => {
                if normal_type == right {
                    Ok(normal_type.clone())
                } else {
                    Err(format!("type mismatch: {:?} vs {:?}", left, right))
                }
            }
        }
    }
}

/// A map that contains all created types.
/// Types should be constructed via the TypeMap,
/// to ensure references to the same type.
pub struct TypeSet {
    arrays: HashSet<Type>,
}

impl TypeSet {
    pub fn new() -> TypeSet {
        let mut type_set = TypeSet{
            arrays: HashSet::new()
        };
        add_builtin_types(&mut type_set);
        return type_set;
    }

    /// Get a genericized array type
    pub fn get_array_type(&mut self, value_type: &Type) -> Type {
        self.arrays.insert(value_type.clone());
        Type::Array(Box::new(value_type.clone()))
    }
}

pub fn add_builtin_types(type_set: &mut TypeSet) {
    type_set.get_array_type(&Type::Byte);
}

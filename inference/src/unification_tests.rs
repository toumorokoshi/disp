use super::unification::*;

#[derive(Clone, PartialEq, Debug)]
enum ExampleTypes {
    Any,
    Bool,
    Array(Box<ExampleTypes>),
}

impl UnificationTypes for ExampleTypes {
    // fn any_type() -> ExampleTypes {
    //     ExampleTypes::Any
    // }
    //
    //     /// returns true if the the type and all subtypes are either
    //     /// a Generic or Any.
    //     fn is_generic_declaration(&self) -> bool {
    //         match self {
    //             ExampleTypes::Any => true,
    //             ExampleTypes::Array(ref typ) => typ.is_generic_declaration(),
    //             _ => false,
    //         }
    //     }

    /// Unify the left and right types. This operation should
    /// perform actions such as:
    /// * choose the more specific type, if there are Any / Generic Types.
    /// * return an error if unification is not possible.
    fn unify(left: &ExampleTypes, right: &ExampleTypes) -> Result<ExampleTypes, String> {
        match left {
            /// Any is the most generic: return the right type if so.
            ExampleTypes::Any => Ok(right.clone()),
            ExampleTypes::Bool => {
                if &ExampleTypes::Bool == right {
                    Ok(ExampleTypes::Bool)
                } else {
                    Err(String::from("unable to unify bools"))
                }
            }
            ExampleTypes::Array(ref typ) => {
                if let ExampleTypes::Array(ref other_type) = right {
                    Ok(ExampleTypes::Array(Box::new(Self::unify(
                        &(*typ),
                        &(*other_type),
                    )?)))
                } else {
                    Err(String::from("unable to unify array types"))
                }
            }
        }
    }
}

#[test]
fn test_unification() {
    let mut type_resolver = TypeResolver::new();
    let a = type_resolver.create_type_var();
    let b = type_resolver.create_type_var();
    let c = type_resolver.create_type_var();
    type_resolver.add_constraint(Constraint::Equality(a.clone(), b.clone()));
    type_resolver.add_constraint(Constraint::IsLiteral(a.clone(), ExampleTypes::Bool));
    assert!(type_resolver.get_type(&a) == Some(ExampleTypes::Bool));
    assert!(type_resolver.get_type(&b) == Some(ExampleTypes::Bool));
    // if there isn't any conflicting information, and the
    // available variables are insufficient, then type inference
    // cannot be performed, and the result should be none.
    assert!(type_resolver.get_type(&c) == None);
}

#[test]
fn test_unification_reverse() {
    let mut type_resolver = TypeResolver::new();
    let a = type_resolver.create_type_var();
    let b = type_resolver.create_type_var();
    let c = type_resolver.create_type_var();
    let d = type_resolver.create_type_var();
    type_resolver.add_constraint(Constraint::IsLiteral(b.clone(), ExampleTypes::Bool));
    type_resolver.add_constraint(Constraint::Equality(a.clone(), b.clone()));
    type_resolver.add_constraint(Constraint::IsLiteral(c.clone(), ExampleTypes::Bool));
    type_resolver.add_constraint(Constraint::Equality(c.clone(), d.clone()));
    assert!(type_resolver.get_type(&a) == Some(ExampleTypes::Bool));
    assert!(type_resolver.get_type(&b) == Some(ExampleTypes::Bool));
    assert!(type_resolver.get_type(&c) == Some(ExampleTypes::Bool));
    assert!(type_resolver.get_type(&d) == Some(ExampleTypes::Bool));
}

/// For parametric polymorphism, type resolvers
/// should handle a generic ANY value, that is considered
/// in typechecks
#[test]
fn test_unification_any_parameter() {
    let mut type_resolver = TypeResolver::new();
    let a = type_resolver.create_type_var();
    type_resolver
        .add_constraint(Constraint::IsLiteral(
            a.clone(),
            ExampleTypes::Array(Box::new(ExampleTypes::Any)),
        ))
        .unwrap();
    type_resolver
        .add_constraint(Constraint::IsLiteral(
            a.clone(),
            ExampleTypes::Array(Box::new(ExampleTypes::Bool)),
        ))
        .unwrap();
    assert!(type_resolver.get_type(&a) == Some(ExampleTypes::Array(Box::new(ExampleTypes::Bool))));
}

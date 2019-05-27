use super::unification::*;

// Types that are passed in for type
// inference may differ than the final type,
// because they do not include type variables.
#[derive(Clone, PartialEq, Debug)]
enum ExampleTypes {
    Any,
    Bool,
    Array,
}

#[test]
fn test_unification_with_generics() {
    let mut type_resolver = TypeResolver::new();
    let a = type_resolver.create_type_var();
    let b = type_resolver.create_type_var();
    type_resolver.add_constraint(Constraint::IsLiteral(
        a.clone(),
        Unresolved::Generic(ExampleTypes::Array, vec![b]),
    ));
    type_resolver.add_constraint(Constraint::IsLiteral(
        b.clone(),
        Unresolved::Literal(ExampleTypes::Bool),
    ));
    assert!(
        type_resolver.get_type(&a)
            == Some(Resolved::Generic(
                ExampleTypes::Array,
                vec![Resolved::Literal(ExampleTypes::Bool)]
            ))
    );
}

#[test]
fn test_unification() {
    let mut type_resolver = TypeResolver::new();
    let a = type_resolver.create_type_var();
    let b = type_resolver.create_type_var();
    let c = type_resolver.create_type_var();
    type_resolver.add_constraint(Constraint::Equality(a.clone(), b.clone()));
    type_resolver.add_constraint(Constraint::IsLiteral(
        a.clone(),
        Unresolved::Literal(ExampleTypes::Bool),
    ));
    assert!(type_resolver.get_type(&a) == Some(Resolved::Literal(ExampleTypes::Bool)));
    assert!(type_resolver.get_type(&b) == Some(Resolved::Literal(ExampleTypes::Bool)));
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
    type_resolver
        .add_constraint(Constraint::IsLiteral(
            b.clone(),
            Unresolved::Literal(ExampleTypes::Bool),
        ))
        .unwrap();
    type_resolver
        .add_constraint(Constraint::Equality(a.clone(), b.clone()))
        .unwrap();
    type_resolver
        .add_constraint(Constraint::IsLiteral(
            c.clone(),
            Unresolved::Literal(ExampleTypes::Bool),
        ))
        .unwrap();
    type_resolver
        .add_constraint(Constraint::Equality(c.clone(), d.clone()))
        .unwrap();
    assert!(type_resolver.get_type(&a) == Some(Resolved::Literal(ExampleTypes::Bool)));
    assert!(type_resolver.get_type(&b) == Some(Resolved::Literal(ExampleTypes::Bool)));
    assert!(type_resolver.get_type(&c) == Some(Resolved::Literal(ExampleTypes::Bool)));
    println!("get type: {:?}", type_resolver.get_type(&d));
    assert!(type_resolver.get_type(&d) == Some(Resolved::Literal(ExampleTypes::Bool)));
}

/// For parametric polymorphism, type resolvers
/// should handle a generic ANY value, that is considered
/// in typechecks
#[test]
fn test_unification_any_parameter() {
    let mut type_resolver = TypeResolver::new();
    let a = type_resolver.create_type_var();
    let b = type_resolver.create_type_var();
    type_resolver
        .add_constraint(Constraint::IsLiteral(a.clone(), Unresolved::Any))
        .unwrap();
    type_resolver
        .add_constraint(Constraint::IsLiteral(
            a.clone(),
            Unresolved::Literal(ExampleTypes::Bool),
        ))
        .unwrap();
    assert!(type_resolver.get_type(&a) == Some(Resolved::Literal(ExampleTypes::Bool)));
}

use super::unification::*;

#[derive(Clone, PartialEq, Debug)]
enum ExampleTypes {
    Bool,
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

//#[test]
//fn test_example_from_paper() {
//    let mut assumptions = AssumptionSet<ExamplesTypes>::new();
//    let t1 = assumptions.create_type_var();
//    let t2 = assumptions.create_type_var();
//    let t3 = assumptions.create_type_var();
//    let t4 = assumptions.create_type_var();
//    let t5 = assumptions.create_type_var();
//    /// ad constraint that y must consume a bool, and that
//    ///
//    assumptions.constraints.extend(vec![
//        Constraint::IsLiteral(t2.clone(),
//        ExamplesTypes::FunctionType(
//            vec![ExampleTypes::Bool],
//            ExampleTypes::TypeVariable(t3.clone()),
//        ]),
//        Constraint::ImplicitInstanceConstraint(t4.clone(), t5.clone(), t3.clone()),
//        Constraint::ImplicitInstanceConstraint(t2.clone(), t5.clone(), t1.clone()),
//        Constraint::Equality(t5.clone(), t1.clone()),
//    ]);
//    let substitution = assumptions.solve();
//    assert!(substitution.get);
//

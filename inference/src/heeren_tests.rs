use super::heeren::{solve_types, Constraint, ConstraintSet};
use std::rc::Rc;

#[derive(Clone, PartialEq, Debug)]
enum ExampleTypes {
    Bool,
    FunctionType(Vec<Box<ExampleTypes>>, Box<ExampleTypes>),
}

#[test]
fn test_unification() {
    let mut constraints = ConstraintSet::new();
    let a = constraints.create_type_var();
    let b = constraints.create_type_var();
    let c = constraints.create_type_var();
    constraints
        .constraints
        .push(Constraint::Equality(a.clone(), b.clone()));
    constraints
        .constraints
        .push(Constraint::IsLiteral(a.clone(), ExampleTypes::Bool));
    let substitution_set = solve_types(&constraints).unwrap();
    assert!(substitution_set.get(&a) == Some(&Some(ExampleTypes::Bool)));
    assert!(substitution_set.get(&b) == Some(&Some(ExampleTypes::Bool)));
    // if there isn't any conflicting information, and the
    // available variables are insufficient, then type inference
    // cannot be performed, and the result should be none.
    assert!(substitution_set.get(&c) == Some(&None));
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

use super::{AssumptionSet, Constraint,SubstitutionSet};

enum ExamplesTypes {
    Bool,
    FunctionType(Vec<Box<ExampleTypes>>, Box<ExamplesTypes>),
    TypeVariable(Rc<TypeVar>),
}

#[test]
fn test_example_from_paper() {
    let mut assumptions = AssumptionSet<ExamplesTypes>::new();
    let t1 = assumptions.create_type_var();
    let t2 = assumptions.create_type_var();
    let t3 = assumptions.create_type_var();
    let t4 = assumptions.create_type_var();
    let t5 = assumptions.create_type_var();
    /// ad constraint that y must consume a bool, and that
    ///
    assumptions.constraints.extend(vec![
        Constraint::IsLiteral(t2.clone(), 
        ExamplesTypes::FunctionType(
            vec![ExampleTypes::Bool],
            ExampleTypes::TypeVariable(t3.clone()),
        ]),
        Constraint::ImplicitInstanceConstraint(t4.clone(), t5.clone(), t3.clone()),
        Constraint::ImplicitInstanceConstraint(t2.clone(), t5.clone(), t1.clone()),
        Constraint::Equality(t5.clone(), t1.clone()),
    ]);
    let substitution = assumptions.solve();
    assert!(substitution.get);
}

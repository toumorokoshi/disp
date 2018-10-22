use super::{AssumptionSet, Constraint,SubstitutionSet};

#[test]
fn test_example_from_paper() {
    let mut assumptions = AssumptionSet::new();
    let m = assumptions.create_type_var();
    let y = assumptions.create_type_var();
    let y_result = assumptions.create_type_var();
    assumptions.add_constraint(Constraint::App
    assumptions.
}

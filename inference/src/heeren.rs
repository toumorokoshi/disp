use std::collections::HashMap;
/// An implementation of Heeren's algorithm for type inference.
use std::fmt::Debug;
use std::rc::Rc;

/// Constraints help deduce the actual type of a type variables.
/// there are a few types.i
#[derive(Clone, PartialEq)]
pub enum Constraint<T>
where
    T: Clone + PartialEq + Debug,
{
    /// specifie that the types introduced must be
    /// equal to each other.
    Equality(Rc<TypeVar>, Rc<TypeVar>),
    // specifies that the LHS must be a generic instance of the RHS.
    IsGeneric(Rc<TypeVar>, Rc<TypeVar>),
    // specifies that the LHS should be the result of generalization the RHS.
    ImplicitInstanceConstraint(Rc<TypeVar>, Rc<TypeVar>),
    /// declares that the type of typevar is of the literal.
    IsLiteral(Rc<TypeVar>, T),
}

/// A TypeVar collects assumptions around this variable
pub type TypeVar = usize;

/// AssumptionSets store the assumptions made around
/// specific type variables.
pub struct ConstraintSet<T>
where
    T: Clone + PartialEq + Debug,
{
    pub constraints: Vec<Constraint<T>>,
    /// all the types that have been instantiated for the assumption set.
    pub types: Vec<Rc<TypeVar>>,
}

/// given an set of assumptions and type variables, return back a substitution set if there
/// are no conflicting constraints
pub fn solve_types<T>(constraints: &ConstraintSet<T>) -> Result<SubstitutionSet<T>, String>
where
    T: Clone + PartialEq + Debug,
{
    // first, we build out a table referencing type variables to a discrete set of constraints.
    // many type variables can be unified in this step.
    let mut constraints_by_type: ConstraintsByType<T> = ConstraintsByType::new();
    // we iterate through all constraints. Specifically for the equality constraint, we unify constraints
    for c in &constraints.constraints {
        match c {
            Constraint::Equality(ref l, ref r) => {
                constraints_by_type.unify(l, r);
            }
            Constraint::IsLiteral(ref var, ref typ) => {
                let constraint_list = constraints_by_type.get_or_create(var);
                constraint_list.push(Constraint::IsLiteral(var.clone(), typ.clone()))
            }
            Constraint::ImplicitInstanceConstraint(ref lef, ref right) => {}
            Constraint::IsGeneric(ref lef, ref right) => {}
        }
    }
    // now we evaluate these constraints.
    let mut type_by_reference = HashMap::new();
    let mut substitution_set = SubstitutionSet::new();
    for type_var in &constraints.types {
        let typ = solve(&mut constraints_by_type, &mut type_by_reference, &type_var)?;
        substitution_set.insert(type_var.clone(), typ);
    }
    Ok(substitution_set)
}

/// given constraitns by type, solve the reference specified.
fn solve<T: Clone + PartialEq + Debug>(
    constraints_by_type: &mut ConstraintsByType<T>,
    type_by_reference: &mut HashMap<usize, T>,
    type_var: &TypeVar,
) -> Result<Option<T>, String> {
    let reference = constraints_by_type
        .reference_by_type
        .get(type_var)
        .unwrap_or(&0);
    // if the calculation has happend already, use it.
    if let Some(ref typ) = type_by_reference.get(reference) {
        return Ok(Some((*typ).clone()));
    }

    let constraints = &constraints_by_type.constraints_by_reference[*reference];
    let mut typ = None;
    for c in constraints {
        match c {
            Constraint::IsLiteral(ref _var, ref literal_type) => {
                if typ == None {
                    typ = Some(literal_type.clone());
                } else {
                    if typ != Some((*literal_type).clone()) {
                        return Err(format!("type mismatch: {:?} and {:?}", typ, literal_type));
                    }
                }
            }
            // TODO: in the future, we need to resolve generics.
            _ => {}
        }
    }
    Ok(typ)
}

pub type ReferenceIndex = usize;

struct ConstraintsByType<T>
where
    T: Clone + PartialEq + Debug,
{
    pub constraints_by_reference: Vec<Vec<Constraint<T>>>,
    pub reference_by_type: HashMap<TypeVar, usize>,
}

impl<T> ConstraintsByType<T>
where
    T: Clone + PartialEq + Debug,
{
    pub fn new() -> ConstraintsByType<T> {
        ConstraintsByType {
            constraints_by_reference: vec![],
            reference_by_type: HashMap::new(),
        }
    }

    pub fn get_or_create(&mut self, var: &TypeVar) -> &mut Vec<Constraint<T>> {
        let type_index = match self.reference_by_type.get(var) {
            Some(i) => i.clone(),
            None => {
                self.constraints_by_reference.push(vec![]);
                self.constraints_by_reference.len() - 1
            }
        };
        self.reference_by_type.insert(var.clone(), type_index);
        return &mut self.constraints_by_reference[type_index];
    }

    /// given two type vars, unify those values and the
    /// corresponding constraints.
    /// this will also remap the constraint indices in
    /// constraint_map.
    pub fn unify(&mut self, left: &TypeVar, right: &TypeVar) {
        // first, we extend the left constraints right the right
        let right_constraints = self.get_or_create(right).to_owned();
        self.get_or_create(left).extend(right_constraints);
        let left_index = match self.reference_by_type.get(left) {
            Some(i) => i.clone(),
            None => panic!("should not be able to reach here, as index should always exist after call to get_or_create"),
        };
        let right_index = match self.reference_by_type.get(right) {
            Some(i) => i.clone(),
            None => panic!("should not be able to reach here, as index should always exist after call to get_or_create"),
        };
        // remap right type to the left index.
        self.reference_by_type.insert(right.clone(), left_index);
        // delete the reference to the right vector.
        self.constraints_by_reference[right_index].clear();
    }
}

impl<T> ConstraintSet<T>
where
    T: Clone + PartialEq + Debug,
{
    pub fn new() -> ConstraintSet<T> {
        ConstraintSet {
            constraints: vec![],
            types: vec![],
        }
    }

    // create a new type variable.
    pub fn create_type_var(&mut self) -> Rc<TypeVar> {
        let var = Rc::new(self.types.len());
        self.types.push(var.clone());
        var
    }
}

/// stores the final result of the type inference algorithm.
pub type SubstitutionSet<T> = HashMap<Rc<TypeVar>, Option<T>>;

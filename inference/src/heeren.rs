use std::collections::HashMap;
/// An implementation of Heeren's algorithm for type inference.
use std::rc::Rc;


/// Constraints help deduce the actual type of a type variables.
/// there are a few types.i
#[derive(Clone)]
pub enum Constraint<T> {
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
pub struct ConstraintSet<T> {
    pub constraints: Vec<Constraint<T>>,
    /// all the types that have been instantiated for the assumption set.
    pub types: Vec<Rc<TypeVar>>,
}

/// given an set of assumptions and type variables, return back a substitution set if there
/// are no conflicting constraints
pub fn solve_types<T>(constraints: &ConstraintSet<T>) -> Result<SubstitutionSet<T>, String> {
    // first, we build out a table referencing type variables to a discrete set of constraints.
    // many type variables can be unified in this step.
    let mut constraints_by_type: ConstraintsByType<T> = ConstraintsByType::new();
    // we iterate through all constraints. Specifically for the equality constraint, we map type variables.
    for c in &constraints.constraints {
        match c {
            Constraint::Equality(ref l, ref r) => {
            },
            Constraint::IsLiteral(ref var, ref typ) => {
                let ref constraint_list = constraints_by_type.get_or_create(var);
            },
            Constraint::ImplicitInstanceConstraint(ref lef, ref right) => {
            },
            Constraint::IsGeneric(ref lef, ref right) => {
            }
        }
    }
    panic!();
}

pub type ReferenceIndex = usize;

struct ConstraintsByType<T> {
    constraint_list: Vec<Vec<Constraint<T>>>,
    constraint_map: HashMap<TypeVar, usize>
}

impl<T> ConstraintsByType<T> {
    pub fn new() -> ConstraintsByType<T> {
        ConstraintsByType {
            constraint_list: vec![],
            constraint_map: HashMap::new()
        }
    }

    pub fn get_or_create(&mut self, var: &TypeVar) -> &Vec<Constraint<T>> {
        let type_index = match self.constraint_map.get(var) {
            Some(i) => i.clone(),
            None => {
                self.constraint_list.push(vec![]);
                self.constraint_list.len() - 1
            }
        };
        return &self.constraint_list[type_index];
    }
}


impl<T> ConstraintSet<T> {
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
pub struct SubstitutionSet<T>(HashMap<Rc<TypeVar>, Option<T>>);

impl<T> SubstitutionSet<T> {
    pub fn new() -> SubstitutionSet<T> {
        SubstitutionSet(HashMap::new())
    }
}

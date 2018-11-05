use std::collections::HashMap;
/// An implementation of Heeren's algorithm for type inference.
use std::rc::Rc;

/// given an assumption set of assumptions, return back a valid substitution set if possible.
pub fn solve_types<T>(assumptions: &AssumptionSet<T>) -> Result<SubstitutionSet<T>, String> {
    panic!();
}

macro_rules! upsert {
    ($set: ident, $x:ident, $y:expr) => {
        if let Some(value) = $set.get_mut($x) {
            value.push($y);
        } else {
            $set.insert($x.clone(), vec![$y]);
        }
    };
}

/// AssumptionSets store the assumptions made around
/// specific type variables.
pub struct AssumptionSet<T> {
    pub constraints: Vec<Constraint<T>>,
    /// all the types that have been instantiated for the assumption set.
    pub types: Vec<Rc<TypeVar>>,
}

impl<T> AssumptionSet<T> {
    pub fn new() -> AssumptionSet<T> {
        AssumptionSet {
            constraints: vec![],
            types: vec![],
        }
    }

    // create a new type variable.
    pub fn create_type_var(&mut self) -> Rc<TypeVar> {
        let var = Rc::new(self.types.len());
        self.types.push(var);
        var.clone()
    }

    /// return a substitution set that satisfies the
    /// constraints provided.
    ///
    /// this method can fail if there are contradicting constraints,
    /// or if there are an insufficient amount of constraints to make a
    /// concrete decision
    pub fn solve(&self) -> SubstitutionSet<T> {
        let mut result_set = SubstitutionSet::new();
        let mut constraint_by_typevar: HashMap<Rc<TypeVar>, Vec<Constraint<T>>> = HashMap::new();
        // first, we run through all constraints, and reorganize so that
        // we order constraints by the typevar we are operating on.
        for ref c in &self.constraints {
            match c {
                Constraint::Equality(ref lhs, ref rhs) => {
                    upsert!(constraint_by_typevar, lhs, *c.clone());
                    upsert!(constraint_by_typevar, rhs, *c.clone());
                },
                Constraint::IsLiteral(ref lhs, ref type) => {
                    upsert!(constraint_by_typevar, lhs, *c.clone());
                },
                Constraint::IsGeneric(ref lhs, ref rhs) => {
                    upsert!(constraint_by_typevar, lhs, *c.clone());
                },
                Constraint::ImplicitInstanceConstraint(ref lhs, ref rhs) => {
                    upsert!(constraint_by_typevar, lhs, *c.clone());
                },
            }
        }
        // once it is organized, we star by solving one variable at a time
        result_set
    }

    /// solve for one specific type. Solve for others if need be.
    fn solve_one_type(constraint_by_typevar: &HashMap<Rc<TypeVar>, Vec<Constraint<T>>>,
                      substitution_set: &mut SubstitutionSet,
                      target: Rc<TypeVar>) {
    )
}

/// stores the final result of the type inference algorithm.
pub struct SubstitutionSet<T>(HashMap<Rc<TypeVar>, Option<T>>);

impl<T> SubstitutionSet<T> {
    pub fn new() -> SubstitutionSet<T> {
        SubstitutionSet(HashMap::new())
    }
}

/// A TypeVar collects assumptions around this variable
pub type TypeVar = usize;

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

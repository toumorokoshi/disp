/// An implementation of Heeren's algorithm for type inference.
use std::rc::Rc;

/// given an assumption set of assumptions, return back a valid substitution set if possible.
pub fn solve_types(assumptions: &AssumptionSet) -> Result<SubstitutionSet, String> {
}

/// AssumptionSets store the assumptions made around
/// specific type variables.
pub struct AssumptionSet {
    pub constraints: Vec<Constraint>,
    /// all the types that have been instantiated for the assumption set.
    pub types: Vec<Rc<TypeVar>>,
}

impl AssumptionSet {
    pub fn new() -> AssumptionSet {
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

    pub fn add_constraint(&mut self, constraint: Constraint) {
    }
}

/// stores the final result of the type inference algorithm.
pub struct SubstitutionSet(HashMap<Rc<TypeVar>>, TypeLiteral);

pub type TypeLiteral = String;

/// A TypeVar collects assumptions around this variable
pub type TypeVar usize;

/// Constraints help deduce the actual type of a type variables.
/// there are a few types.
pub enum Constraint {
    /// specifie that the types introduced must be
    /// equal to each other.
    Equality(Rc<TypeVar>, Rc<TypeVar>),
    // specifies that the LHS must be a generic instance of the RHS.
    IsGeneric(Rc<TypeVar>, Rc<TypeVar>),
    // specifies that the LHS should be the result of generalization the RHS.
    ImplicitInstanceConstraint(Rc<TypeVar>, Rc<TypeVar>),
    /// declares that the type of typevar is of the literal.
    IsLiteral(Rc<TypeVar>, TypeLiteral),
    }


pub enum Expression {
    Literal(TypeLiteral),
    // TODO: seems like there needs to be a way to express a list of types,
    // otherwise it's not possible to define something like a function signature.
}

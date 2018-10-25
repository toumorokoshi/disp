/// An implementation of Heeren's algorithm for type inference.
use std::rc::Rc;

/// given an assumption set of assumptions, return back a valid substitution set if possible.
pub fn solve_types(assumptions: &AssumptionSet) -> Result<SubstitutionSet, String> {
}


macro_rules! upsert {
    ($x:ident, $y:expr) => {
    }
}

/// AssumptionSets store the assumptions made around
/// specific type variables.
pub struct AssumptionSet<T> {
    pub constraints: Vec<Constraint<T>>,
    /// all the types that have been instantiated for the assumption set.
    pub types: Vec<Rc<TypeVar>>,
}

impl AssumptionSet<T> {
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
    pub solve(&self) -> SubstitutionSet<T> {
        let result_set = SubstitutionSet::new();
        let mut constraint_by_typevar = HashMap::new();
        // first, we run through all constraints, and reorganize so that
        // we order constraints by the typevar we are operating on.
        for ref c in &self.constraints {
            match c {
                Constraint::Equality(ref lhs, ref rhs) => {
                }
            }
        }
   }
}

/// stores the final result of the type inference algorithm.
pub struct SubstitutionSet<T>(HashMap<Rc<TypeVar>>, Option<T>);

/// A TypeVar collects assumptions around this variable
pub type TypeVar usize;

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


pub enum Expression {
    Literal(TypeLiteral),
    App(Box<Expression>, Box<Expression>),
    // TODO: seems like there needs to be a way to express a list of types,
    // otherwise it's not possible to define something like a function signature.
}

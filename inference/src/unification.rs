use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

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

/// The TypeResolver is an iterative resolver
/// of type variables. The TypeResolver immediately
/// evaluates conditions and provides results and errors.
pub struct TypeResolver<T>
where
    T: Clone + PartialEq + Debug,
{
    type_vars: Vec<Rc<TypeVar>>,
    /// a counter to monotonically iterate reference
    /// count.
    reference_counter: usize,
    constraints_by_reference: HashMap<usize, Vec<Constraint<T>>>,
    reference_by_typevar: HashMap<Rc<TypeVar>, usize>,
    type_by_reference: HashMap<usize, T>,
}

impl<T: Clone + PartialEq + Debug> TypeResolver<T> {
    pub fn new() -> TypeResolver<T> {
        TypeResolver {
            type_vars: vec![],
            reference_counter: 0,
            constraints_by_reference: HashMap::new(),
            reference_by_typevar: HashMap::new(),
            type_by_reference: HashMap::new(),
        }
    }

    /// allocate a new type variable,
    /// which can be used in constraint relations.
    pub fn create_type_var(&mut self) -> Rc<TypeVar> {
        let var = Rc::new(self.type_vars.len());
        self.type_vars.push(var.clone());
        var
    }

    /// add a constraint to the set. Evaluate the constraint
    /// and resolve any type variables that can now be resolved
    /// with the new constraint provided.
    pub fn add_constraint(&mut self, c: Constraint<T>) -> Result<(), String> {
        match c {
            Constraint::Equality(ref l, ref r) => {
                /// unify the two references.
                let left_index = self.get_or_create_reference(l);
                let right_index = self.get_or_create_reference(r);
                self.reference_by_typevar
                    .insert(l.clone(), right_index.clone());
            }
            Constraint::IsLiteral(ref type_var, ref typ) => {
                let reference = self.get_or_create_reference(type_var);
                self.set_type(reference, typ.clone())?;
            }
            // TODO: fully enumerate all constraints.
            _ => {}
        }
        Ok(())
    }

    pub fn get_type(&self, t: &TypeVar) -> Option<T> {
        println!("{:?}", self.reference_by_typevar);
        println!("{:?}", self.type_by_reference);
        match self.reference_by_typevar.get(t) {
            Some(reference) => match self.type_by_reference.get(reference) {
                Some(t) => Some(t.clone()),
                None => None,
            },
            None => None,
        }
    }

    fn get_or_create_reference(&mut self, t: &Rc<TypeVar>) -> usize {
        self.reference_by_typevar
            .entry(t.clone())
            .or_insert({
                self.reference_counter += 1;
                self.reference_counter
            })
            .clone()
    }

    fn set_type(&mut self, reference: usize, typ: T) -> Result<(), String> {
        let current_value = self
            .type_by_reference
            .entry(reference)
            .or_insert(typ.clone());
        if *current_value != typ {
            return Err(format!(
                "type collision! Literal already determined as {:?}, but found {:?}",
                current_value, typ
            ));
        }
        Ok(())
    }
}

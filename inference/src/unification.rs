use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub enum Constraint<T>
where
    T: Clone + PartialEq + Debug,
{
    /// specifie that the types introduced must be
    /// equal to each other.
    Equality(TypeVar, TypeVar),
    // specifies that the LHS must be a generic instance of the RHS.
    IsGeneric(TypeVar, TypeVar),
    // specifies that the LHS should be the result of generalization the RHS.
    ImplicitInstanceConstraint(TypeVar, TypeVar),
    /// declares that the type of typevar is of the literal.
    IsLiteral(TypeVar, T),
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
    type_vars: Vec<TypeVar>,
    /// a counter to monotonically iterate reference
    /// count.
    reference_counter: usize,
    reference_by_typevar: HashMap<TypeVar, usize>,
    type_by_reference: HashMap<usize, T>,
}

impl<T: Clone + PartialEq + Debug> TypeResolver<T> {
    pub fn new() -> TypeResolver<T> {
        TypeResolver {
            type_vars: vec![],
            reference_counter: 0,
            reference_by_typevar: HashMap::new(),
            type_by_reference: HashMap::new(),
        }
    }

    /// allocate a new type variable,
    /// which can be used in constraint relations.
    pub fn create_type_var(&mut self) -> TypeVar {
        let var = self.type_vars.len();
        self.type_vars.push(var.clone());
        var
    }

    /// add a constraint to the set. Evaluate the constraint
    /// and resolve any type variables that can now be resolved
    /// with the new constraint provided.
    pub fn add_constraint(&mut self, c: Constraint<T>) -> Result<(), String> {
        match c {
            Constraint::Equality(ref l, ref r) => match self.reference_by_typevar.get(l).clone() {
                None => {
                    let right_index = self.get_or_create_reference(r);
                    self.reference_by_typevar.insert(l.clone(), right_index);
                }
                Some(left_index) => match self.reference_by_typevar.get(r).clone() {
                    None => {
                        self.reference_by_typevar
                            .insert(r.clone(), left_index.clone());
                    }
                    Some(right_index) => {
                        let left_type = self.type_by_reference.get(right_index).clone();
                        let right_type = self.type_by_reference.get(right_index).clone();
                        if left_type != right_type {
                            return Err(String::from(
                                "type mismatch when trying to add constraint.",
                            ));
                        }
                    }
                },
            },
            Constraint::IsLiteral(ref type_var, ref typ) => {
                let reference = self.get_or_create_reference(type_var);
                self.set_type(reference, typ.clone())?;
            }
            // TODO: fully enumerate all constraints.
            _ => {}
        }
        Ok(())
    }

    /// return true if the type vars are referencing the
    /// same variable.
    pub fn is_equal(&mut self, l: &TypeVar, r: &TypeVar) -> bool {
        self.get_or_create_reference(l) == self.get_or_create_reference(r)
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

    fn get_or_create_reference(&mut self, t: &TypeVar) -> usize {
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

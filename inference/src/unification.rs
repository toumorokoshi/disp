use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub enum Unresolved<T> {
    Any,
    Literal(T),
    Generic(T, Vec<TypeVar>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Resolved<T> {
    Literal(T),
    Generic(T, Vec<Resolved<T>>),
}

#[derive(Clone, PartialEq)]
pub enum Constraint<T>
where
    T: Clone + PartialEq + Debug,
{
    /// specifie that the types introduced must be
    /// equal to each other.
    Equality(TypeVar, TypeVar),
    /// declares that the type of typevar is of the literal.
    IsLiteral(TypeVar, Unresolved<T>),
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
    type_by_reference: HashMap<usize, Unresolved<T>>,
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
            Constraint::Equality(ref l, ref r) => {
                let reference = match self.reference_by_typevar.get(l) {
                    Some(left_index) => Some(left_index.clone()),
                    None => None,
                };
                match reference {
                    None => {
                        let right_index = self.get_or_create_reference(r);
                        self.reference_by_typevar.insert(l.clone(), right_index);
                    }
                    Some(left_index) => {
                        let right_reference = match self.reference_by_typevar.get(r) {
                            Some(right_index) => Some(right_index.clone()),
                            None => None,
                        };
                        match right_reference {
                            None => {
                                self.reference_by_typevar
                                    .insert(r.clone(), left_index.clone());
                            }
                            Some(right_index) => {
                                let left = self.type_by_reference.get(&left_index).unwrap().clone();
                                let right =
                                    self.type_by_reference.get(&right_index).unwrap().clone();
                                let unified_type = self.unify(&left, &right)?;
                                self.type_by_reference
                                    .insert(left_index, unified_type.clone());
                                self.type_by_reference.insert(right_index, unified_type);
                            }
                        }
                    }
                }
            }
            Constraint::IsLiteral(ref type_var, ref typ) => {
                let reference = self.get_or_create_reference(type_var);
                self.set_type(reference, typ.clone())?;
            }
        }
        Ok(())
    }

    /// return true if the type vars are referencing the
    /// same variable.
    pub fn is_equal(&mut self, l: &TypeVar, r: &TypeVar) -> bool {
        self.get_or_create_reference(l) == self.get_or_create_reference(r)
    }

    fn unify(
        &mut self,
        left: &Unresolved<T>,
        right: &Unresolved<T>,
    ) -> Result<Unresolved<T>, String> {
        match left {
            // Any is the most generic: return the right type if so.
            Unresolved::Any => Ok(right.clone()),
            Unresolved::Literal(ref left_type) => {
                if let Unresolved::Literal(ref right_type) = right {
                    if left_type == right_type {
                        return Ok(Unresolved::Literal(left_type.clone()));
                    }
                }
                return Err(String::from("unable to unify literal with non-literal"));
            }
            Unresolved::Generic(ref left_type, ref left_subtypes) => match &right {
                Unresolved::Generic(ref right_type, ref right_subtypes) => {
                    if left_subtypes.len() != right_subtypes.len() {
                        return Err(String::from("unable to unify literal with non-literal"));
                    }
                    if right_type != left_type {
                        return Err(String::from("generic type mismatch"));
                    }
                    for i in 0..left_subtypes.len() {
                        self.add_constraint(Constraint::Equality(
                            left_subtypes[i],
                            right_subtypes[i],
                        ))?;
                    }
                    Ok(Unresolved::Generic(
                        left_type.clone(),
                        left_subtypes.clone(),
                    ))
                }
                _ => Err(String::from("unable to unify literal with non-literal")),
            },
        }
    }

    /// return the resolved type for the type variable, if it exists.
    /// In the case of a generic, a tuple will be returned with the
    /// list of subtypes, resolved as well.
    pub fn get_type(&self, t: &TypeVar) -> Option<Resolved<T>> {
        println!("{:?}", self.reference_by_typevar);
        println!("{:?}", self.type_by_reference);
        match self.reference_by_typevar.get(t) {
            Some(reference) => match self.type_by_reference.get(reference) {
                Some(type_resolver_type) => match type_resolver_type {
                    Unresolved::Any => None,
                    Unresolved::Literal(ref t) => Some(Resolved::Literal(t.clone())),
                    Unresolved::Generic(ref t, ref subtypes) => {
                        let mut resolved_subtypes = Vec::with_capacity(subtypes.len());
                        for subtype in subtypes {
                            match self.get_type(subtype) {
                                Some(s) => resolved_subtypes.push(s),
                                None => return None,
                            }
                        }
                        Some(Resolved::Generic(t.clone(), resolved_subtypes))
                    }
                },
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

    fn set_type(&mut self, reference: usize, typ: Unresolved<T>) -> Result<(), String> {
        let existing_type = match self.type_by_reference.get(&reference) {
            Some(current_typ) => current_typ.clone(),
            None => typ.clone(),
        };
        let unified_type = self.unify(&existing_type, &typ)?;
        self.type_by_reference.insert(reference, unified_type);
        Ok(())
    }
}

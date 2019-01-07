use super::{Compiler, DispError, DispResult, FunctionMap, Token, Type, UnparsedFunction};
use inference::{Constraint, TypeResolver, TypeVar};
use std::{collections::HashMap, rc::Rc};

/// The result of the type annotation phase is a map
/// of specialized functions, with their discrete types.
pub type AnnotatedFunctionMap = HashMap<String, HashMap<Vec<Type>, AnnotatedFunction>>;

pub struct AnnotatedFunction {
    pub function: Rc<UnparsedFunction>,
    pub arg_types: Vec<Type>,
    pub return_type: Type,
}

/// Internal data structure to keep
/// track of all of the annotated functions.
struct TypevarFunctionMap {
    pub map: HashMap<String, HashMap<Vec<TypeVar>, TypevarFunction>>,
}

impl TypevarFunctionMap {
    pub fn new() -> TypevarFunctionMap {
        TypevarFunctionMap {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, args: Vec<TypeVar>, function: TypevarFunction) {
        let function_by_typevar = self.map.entry(name).or_insert(HashMap::new());
        function_by_typevar.insert(args, function);
    }

    pub fn get(&self, name: &String, args: &Vec<TypeVar>) -> Option<&TypevarFunction> {
        match self.map.get(name) {
            None => None,
            Some(ref function_by_typevar) => match function_by_typevar.get(args) {
                None => None,
                Some(ref function) => Some(function),
            },
        }
    }
}

/// Annotated functions have Type variables attached,
/// but have not yet been resolved. This phase should ensure
/// discrete types on all functions, thus should only be used
/// internally.
struct TypevarFunction {
    pub function: Rc<UnparsedFunction>,
    pub arg_types: Vec<TypeVar>,
    pub return_type: TypeVar,
}

impl TypevarFunction {
    pub fn to_annotated_function(
        &self,
        type_resolver: &TypeResolver<Type>,
    ) -> DispResult<AnnotatedFunction> {
        let return_type = match type_resolver.get_type(&self.return_type) {
            Some(t) => t,
            None => return Err(DispError::new("unable to resolve type variable")),
        };
        let arg_types = {
            let mut arg_types = vec![];
            for type_var in &self.arg_types {
                let typ = match type_resolver.get_type(type_var) {
                    Some(t) => t,
                    None => return Err(DispError::new("unable to resolve type variable")),
                };
                arg_types.push(typ);
            }
            arg_types
        };
        Ok(AnnotatedFunction {
            function: self.function.clone(),
            return_type: return_type,
            arg_types: arg_types,
        })
    }
}

/// Annotate types that may or may not have been resolved.
/// Type annotation works with on-the-fly unification using the
/// inference library.
///
/// Type annotation includes additional complexity, as the prototype function
/// may be specialized for multiple different type signatures.
/// The returned AnnotatedFunctionMap includes mappings for multiple return
/// types.
pub fn annotate_types(
    compiler: &mut Compiler,
    functions: &FunctionMap,
) -> DispResult<AnnotatedFunctionMap> {
    let mut type_resolver = TypeResolver::new();
    let mut annotated_functions = TypevarFunctionMap::new();
    // as all functions can have untyped arguments, we should
    // start with the code that will actually be executed. i.e. main
    // functions only.
    // TODO: have a more robust way to detect main functions.
    for (name, function) in functions {
        if name.contains("main") {
            annotate_token(
                compiler,
                &functions,
                &mut type_resolver,
                &mut annotated_functions,
                &function.body,
            );
        }
    }
    // after this point. we have in annotated_functions all methods that are actually
    // invoked. We can now convert those into annotated types with concrete
    // type variables.
    let mut result = AnnotatedFunctionMap::new();
    for (name, function_by_args) in &annotated_functions.map {
        for typevar_function in function_by_args.values() {
            let annotated_function = typevar_function.to_annotated_function(&type_resolver)?;
            result
                .entry(name.clone())
                .or_insert(HashMap::new())
                .insert(annotated_function.arg_types.clone(), annotated_function);
        }
    }
    Ok(result)
}

fn annotate_token(
    compiler: &mut Compiler,
    functions: &FunctionMap,
    types: &mut TypeResolver<Type>,
    annotated_functions: &mut TypevarFunctionMap,
    token: &Token,
) -> DispResult<TypeVar> {
    let type_var = types.create_type_var();
    match token {
        Token::List(ref token_list) => {
            for t in token_list {
                annotate_token(compiler, functions, types, annotated_functions, t);
            }
        }
        Token::Expression(ref expression) => {
            parse_and_add_expression(compiler, functions, types, annotated_functions, expression)?;
        }
        Token::Integer(_) => {
            types.add_constraint(Constraint::IsLiteral(type_var.clone(), Type::Int));
        }
        Token::Boolean(_) => {
            types.add_constraint(Constraint::IsLiteral(type_var.clone(), Type::Bool));
        }
        Token::String(_) => {
            types.add_constraint(Constraint::IsLiteral(type_var.clone(), Type::String));
        }
        Token::Map(_) => {
            // types.add_constraint(Constraint::IsLiteral(type_var.clone(), Type::Map<Type::String, Type::String>));
        }
        _ => {}
    }
    Ok(type_var)
}

fn parse_and_add_expression(
    compiler: &mut Compiler,
    functions: &FunctionMap,
    types: &mut TypeResolver<Type>,
    annotated_functions: &mut TypevarFunctionMap,
    expression: &Vec<Token>,
) -> DispResult<TypeVar> {
    if let Token::Symbol(name) = expression[0].clone() {
        let arg_type_variables = {
            let mut arg_type_variables = vec![];
            for token in &expression[1..] {
                arg_type_variables.push(annotate_token(
                    compiler,
                    functions,
                    types,
                    annotated_functions,
                    token,
                )?);
            }
            arg_type_variables
        };
        // first, we should check the compiler to see if
        // there is a matching primitive function.
        // TODO: check types in compiler
        if let Some(expression) = compiler.data.builtin_expressions.get(&*name) {
            return (expression.typecheck)(types, &arg_type_variables);
        }
        // next, we check if there is an already
        // parsed function that matches the type signature
        if let Some(ref function) = annotated_functions.get(&*name, &arg_type_variables) {
            return Ok(function.return_type);
        }
        // TODO: parse already generated functions.
        // finally, we check to see if there is an unparsed function with the name
        // and argument count, and if so we start generating and expression for that.
        match functions.get(&*name) {
            None => Err(DispError::new(&format!(
                "unable to find function with name {}",
                *name
            ))),
            Some(ref function) => {
                let return_type = types.create_type_var();
                let typevar_function = TypevarFunction {
                    function: (*function).clone(),
                    arg_types: arg_type_variables.clone(),
                    return_type: return_type.clone(),
                };
                // the annotated function must be inserted before parsing the body,
                // to ensure that recursive definitions to not re-enter this and cause
                // a recursive loop.
                annotated_functions.insert(*name, arg_type_variables, typevar_function);
                Ok(return_type)
            }
        }
    } else {
        Err(DispError::new(&format!(
            "expected symbol as first argument to expression, found {}",
            &expression[0]
        )))
    }
}

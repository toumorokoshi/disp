mod parser;

pub use self::parser::parse_macro;
use super::{DispError, DispResult, FunctionMap, Token, UnparsedFunction};
use std::{collections::HashMap, rc::Rc};

/// Macros represent functions that execute
/// compile time and return back additional syntax blocks to
/// be evaluated.
#[derive(Clone, Debug)]
pub struct Macro {
    pub arguments: Vec<String>,
    pub body: Token,
}

pub type MacroMap = HashMap<String, Macro>;

/// modify function map in place, using macros to expand.
pub fn apply_macros_to_function_map(
    macros: &MacroMap,
    functions: &mut FunctionMap,
) -> DispResult<()> {
    for value in functions.values_mut() {
        if let Some(ref mut function) = Rc::get_mut(value) {
            apply_macros_to_function(macros, function)?;
        }
    }
    Ok(())
}

fn apply_macros_to_function(macros: &MacroMap, function: &mut UnparsedFunction) -> DispResult<()> {
    function.body = apply_macros_to_token(macros, &mut function.body)?;
    Ok(())
}

fn apply_macros_to_token(macros: &MacroMap, token: &mut Token) -> DispResult<Token> {
    // TODO: figure out how return back original expressions without cloning values
    match token {
        Token::Expression(e) => return expand_expression(macros, e.clone()),
        Token::List(ref mut list) => {
            for i in 0..list.len() {
                list[i] = apply_macros_to_token(macros, &mut list[i])?;
            }
        }
        Token::Map(ref mut m) => {
            for token in m.values_mut() {
                *token = apply_macros_to_token(macros, &mut *token)?;
            }
        }
        _ => {}
    };
    Ok(token.clone())
}

/// Return the expanded token if an expansion was performed. Otherwise
/// return None
fn expand_expression(macros: &MacroMap, mut expression: Vec<Token>) -> DispResult<Token> {
    let expression_length = expression.len();
    for i in 0..expression_length {
        expression[i] = apply_macros_to_token(macros, &mut expression[i])?;
    }
    if let Some((func_token, args)) = expression.split_first() {
        if let Token::Symbol(ref s) = func_token {
            if let Some(macro_instance) = macros.get(&**s) {
                return Ok(expand_macro(macro_instance, args)?);
            }
        }
    }
    Ok(Token::Expression(expression))
}

pub fn expand_macro(disp_macro: &Macro, args: &[Token]) -> DispResult<Token> {
    let mut replacement_tokens = HashMap::new();
    if disp_macro.arguments.len() != args.len() {
        return Err(DispError::new(&format!(
            "expected {} arguments for macro. found {:?}",
            disp_macro.arguments.len(),
            args
        )));
    }
    for i in 0..disp_macro.arguments.len() {
        replacement_tokens.insert(disp_macro.arguments[i].clone(), args[i].clone());
    }
    let result = expand_token(&replacement_tokens, &disp_macro.body);
    Ok(result)
}

// given a token, substitute any values
// in the macro with the appropriate values.
pub fn expand_token(replacement_tokens: &HashMap<String, Token>, token: &Token) -> Token {
    match token {
        &Token::Symbol(ref s) => match replacement_tokens.get(&(*s.clone())) {
            Some(t) => t.clone(),
            None => Token::Symbol(Box::new(*s.clone())),
        },
        &Token::List(ref token_list) => {
            let mut new_list = vec![];
            for t in token_list {
                new_list.push(expand_token(replacement_tokens, t));
            }
            Token::List(new_list)
        }
        &Token::Expression(ref token_list) => {
            let mut new_list = vec![];
            for t in token_list {
                new_list.push(expand_token(replacement_tokens, t));
            }
            Token::Expression(new_list)
        }
        &Token::Map(ref map) => {
            let mut result_map = HashMap::new();
            for (key, value) in map.iter() {
                result_map.insert(key.clone(), expand_token(replacement_tokens, value));
            }
            Token::Map(Box::new(result_map))
        }
        t => t.clone(),
    }
}

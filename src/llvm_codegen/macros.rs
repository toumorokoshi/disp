use super::{CodegenError, CodegenResult, Context, Object, Token};
use std::collections::HashMap;

/// Macros represent macros in disp: functions that execute
/// compile time and return back additional syntax blocks to
/// be evaluated.
#[derive(Clone)]
pub struct Macro {
    pub arguments: Vec<String>,
    pub body: Token,
}

pub fn expand_macro<'b, 'c>(
    context: &mut Context<'b, 'c>,
    disp_macro: &Macro,
    args: &[Token],
) -> CodegenResult<Token> {
    let mut replacement_tokens = HashMap::new();
    if disp_macro.arguments.len() != args.len() {
        return Err(CodegenError::new(&format!(
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

pub fn build_macro<'a, 'b, 'c>(
    context: &'a mut Context<'b, 'c>,
    args: &'a [Token],
) -> CodegenResult<Object> {
    if args.len() == 3 {
        if let (&Token::Symbol(ref name), &Token::List(ref token_list)) = (&args[0], &args[1]) {
            let mut arguments = vec![];
            for t in token_list {
                if let Token::Symbol(ref arg_name) = t {
                    arguments.push(*arg_name.clone());
                } else {
                    return Err(CodegenError::new(&format!(
                        "macro variables names should be symbols. found {}",
                        t,
                    )));
                }
            }
            context.scope.macros.insert(
                *name.clone(),
                Macro {
                    arguments: arguments,
                    body: args[2].clone(),
                },
            );
            return Ok(Object::none());
        }
    }
    Err(CodegenError::new(&format!(
        "a macro requires three arguments: name, variables, body. found {:?}",
        args
    )))
}

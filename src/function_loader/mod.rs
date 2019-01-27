use super::{Compiler, DispError, DispResult, MacroMap, Token};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct UnparsedFunction {
    pub args: Vec<Token>,
    pub body: Token,
}

impl UnparsedFunction {
    pub fn new(args: Vec<Token>, body: Token) -> UnparsedFunction {
        return UnparsedFunction { args, body };
    }
}

/// A FunctionMap of string to unparsed functions.
/// The UnparsedFunction is reference counted because it
/// is eventually spread across multiple specialized functions
/// definitions in the future.
pub type FunctionMap = HashMap<String, Rc<UnparsedFunction>>;

/// consume tokens, subdividing them into function and macro declarations.
pub fn parse_functions_and_macros(
    compiler: &mut Compiler,
    parent_token: Token,
) -> DispResult<(FunctionMap, MacroMap)> {
    let mut function_map = HashMap::new();
    // instructions that are not a part of any function
    // are automatically added to the main function.
    let mut main_function_body = vec![];
    if let Token::List(tokens) = parent_token {
        for token in tokens {
            match token {
                // the only token we really need to parse out is the expression,
                // since that's the only thing that can define a top-level function.
                // everything else is part of the main function.
                Token::Expression(e) => match e[0].clone() {
                    Token::Symbol(ref s) => {
                        if **s == "fn" {
                            let (name, function) = parse_function(e)?;
                            function_map.insert(name, function);
                        } else {
                            main_function_body.push(Token::Expression(e));
                        }
                    }
                    _ => main_function_body.push(Token::Expression(e)),
                },
                t => main_function_body.push(t),
            }
        }
    }
    function_map.insert(
        String::from("main"),
        Rc::new(UnparsedFunction::new(
            vec![],
            Token::List(main_function_body),
        )),
    );
    Ok((function_map, HashMap::new()))
}

fn parse_function(tokens: Vec<Token>) -> DispResult<(String, Rc<UnparsedFunction>)> {
    if tokens.len() != 4 {
        return Err(DispError::new(&format!(
            "A function declaration should have 4 tokens: fn <name> <args> <body>. found {} for {:?}",
            tokens.len(),
            tokens
        )));
    }
    let name = {
        if let Token::Symbol(ref s) = tokens[1] {
            s.clone()
        } else {
            return Err(DispError::new(&format!(
                "function name must be a symbol, found {}",
                &tokens[1]
            )));
        }
    };
    if cfg!(feature = "debug") {
        println!("parse function: {}", &name);
    }
    if *name == "main" {
        return Err(DispError::new("unable to name function main"));
    }
    let args = {
        if let Token::List(ref list) = tokens[2] {
            list.clone()
        } else {
            return Err(DispError::new(&format!(
                "function args must be a list of symbols, found {}",
                &tokens[2]
            )));
        }
    };
    return Ok((
        *name,
        Rc::new(UnparsedFunction::new(args, tokens[3].clone())),
    ));
}

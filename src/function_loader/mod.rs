use super::{Compiler, Token};
use std::collections::HashMap;

pub struct FunctionMap {
}

pub struct Function {
    pub name: String,
    pub body: Vec<Token>,
}

impl Function {
    pub fn new(name: String) ->  Function {
        return Function{
            name
        };
    }
}


/// consume tokens, subdividing them into function and macro declarations.
pub fn parse_functions_and_macros(compiler: &mut Compiler, token: Token) {
    let map = HashMap::new();
    // instructions that are not a part of any function
    // are automatically added to the main function.
    let mut main_function = Function::new(String::from("main"));
    match Token {
        // the only token we really need to parse out is the expression,
        // since that's the only thing that can define a top-level function.
        // everything else is part of the main function.
        Expression => {
        },
        _(t) => main_function.body.push(t),
    }
}

fn parse_token(copmiler: &mut Compiler, token: &Token) {
    match token {
    }
}
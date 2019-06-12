use super::{DispError, DispResult, Macro, Token};

pub fn parse_macro(args: Vec<Token>) -> DispResult<(String, Macro)> {
    if args.len() == 4 {
        if let (&Token::Symbol(ref name), &Token::List(ref token_list)) = (&args[1], &args[2]) {
            let mut arguments = vec![];
            for t in token_list {
                if let Token::Symbol(ref arg_name) = t {
                    arguments.push(*arg_name.clone());
                } else {
                    return Err(DispError::new(&format!(
                        "macro variables names should be symbols. found {}",
                        t,
                    )));
                }
            }
            return Ok((
                (**name).clone(),
                Macro {
                    arguments: arguments,
                    body: args[3].clone(),
                },
            ));
        }
    }
    Err(DispError::new(&format!(
        "a macro requires three arguments: name, variables, body. found {:?}",
        args
    )))
}

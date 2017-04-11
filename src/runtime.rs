use std::collections::HashMap;
use super::ast::{Token, ensure_symbol};
use super::core::{Block, DFunc};

pub fn eval(block: &mut Block, token: &Token) -> Token {
    match token {
        &Token::Expression(ref tl) => eval_expr(block, tl),
        &Token::List(ref tl) => eval_list(block, tl),
        &Token::Symbol(ref s) => Token::Symbol(s.clone()),
        &Token::BangSymbol(ref s) => Token::BangSymbol(s.clone()),
        &Token::Integer(i) => Token::Integer(i),
        &Token::Boolean(b) => Token::Boolean(b),
        &Token::None => Token::None
    }
}

pub fn eval_expr(block: &mut Block, statement: &[Token]) -> Token {
    let mut func: Option<DFunc> = None;
    if let Some((func_token, args)) = statement.split_first() {
        {
            let func_name = ensure_symbol(func_token);
            let res = block.locals.get(func_name).clone();
            if let Some(ref f) = res {
                func = Some(*f.clone());
            }
        }
        if let Some(f) = func {
            return f(block, args);
        }
    }
    return Token::None;
}

pub fn eval_list(block: &mut Block, list: &[Token]) -> Token {
    let mut result = Vec::new();
    for e in list {
        result.push(eval(block, e));
    }
    return Token::List(result);
}

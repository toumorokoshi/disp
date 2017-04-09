use std::collections::HashMap;
use super::ast::Token;

type DFunc = fn(&mut Block, &[Token]) -> Token;

pub fn eval(block: &mut Block, token: &Token) -> Token {
    match token {
        &Token::List(ref tl) => eval_expr(block, tl),
        &Token::Symbol(ref s) => Token::Symbol(s.clone()),
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

pub struct Block {
    locals: HashMap<String, DFunc>
}

impl Block {
    pub fn new() -> Block {
        let mut block = Block {
            locals: HashMap::new(),
        };
        block.locals.insert(String::from("+"), plus as DFunc);
        block.locals.insert(String::from("if"), if_expr as DFunc);
        return block;
    }
}

fn plus(block: &mut Block, args: &[Token]) -> Token {
    let left_op = ensure_int(block, &args[0]);
    let right_op = ensure_int(block, &args[1]);
    return Token::Integer(left_op + right_op);
}

fn if_expr(block: &mut Block, args:  &[Token]) -> Token {
    let condition_result = eval(block, &args[0]);
    if let Token::Boolean(b) = condition_result {
        if b {
            if let &Token::List(ref tl) = &args[1] {
                return eval_expr(block, &tl);
            }
        } else {
            return Token::None;
        }
    }
    panic!("incorrect if arguments");
}

fn ensure_symbol<'a>(t: &'a Token) -> &'a str {
    if let &Token::Symbol(ref s) = t {
        return s;
    }
    panic!("string token expected.");
}

fn ensure_int(block: &mut Block, t: &Token) -> i64 {
    let eval_t = eval(block, t);
    if let Token::Integer(i) = eval_t {
        return i;
    }
    panic!("intn token expected.");
}

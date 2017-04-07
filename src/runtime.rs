use std::collections::HashMap;
use super::ast::Token;

type DFunc = fn(&mut Block, &[Token]) -> Token;

/*
pub fn eval(args: Vec<Token>) -> Vec<String> {
    let mut block = Block::new();
    eval_block(&mut block, &args)
}

fn eval_block(block: &mut Block, statements: &Vec<Vec<Token>>) -> Vec<String> {
    let mut result = Vec::new();
    for statement in statements {
        result = eval_expr(block, statement)
    }
    return result;
}
*/

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
        return block;
    }
}

fn plus(block: &mut Block, args: &[Token]) -> Token {
    let left_op = ensure_int(&args[0]);
    let right_op = ensure_int(&args[1]);
    return Token::Integer(left_op + right_op);
}

/*
fn if_expr(block: &mut Block, args:  &[Token]) -> Vec<String> {
    if let &Token::List(tl) = &args[0] {
        let condition_result = eval_expr(block, tl);
    }
    panic!("incorrect if arguments");
}
*/

fn ensure_symbol<'a>(t: &'a Token) -> &'a str {
    if let &Token::Symbol(ref s) = t {
        return s;
    }
    panic!("string token expected.");
}

fn ensure_int(t: &Token) -> i64 {
    if let &Token::Integer(i) = t {
        return i;
    }
    panic!("intn token expected.");
}

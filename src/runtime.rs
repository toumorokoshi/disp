use std::collections::HashMap;
use super::ast::Token;

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

pub fn eval_expr(block: &mut Block, statement: &[Token]) -> Vec<String> {
    if let Some((func_token, args)) = statement.split_first() {
        let func_name = ensure_symbol(func_token);
        match block.locals.get(func_name) {
            Some(f) => {
                return f(args);
            },
            None => {}
        }
    }
    return Vec::new();
}

pub struct Block {
    locals: HashMap<String, fn(&[Token]) -> Vec<String>>
}

impl Block {
    pub fn new() -> Block {
        let mut block = Block {
            locals: HashMap::new(),
        };
        block.locals.insert(String::from("+"), plus as fn(&[Token]) -> Vec<String>);
        return block;
    }
}

fn plus(args: &[Token]) -> Vec<String> {
    let left_op = ensure_int(args[0]);
    let right_op = ensure_int(args[1]);
    let mut return_value = Vec::new();
    return_value.push((left_op + right_op).to_string());
    return return_value;
}

fn ensure_symbol<'a>(t: Token) -> &'a str {
    if let Token::Symbol(s) = t {
        return s;
    }
    panic!("string token expected.");
}

fn ensure_int(t: Token) -> i64 {
    if let Token::Integer(i) = t {
        return i;
    }
    panic!("intn token expected.");
}

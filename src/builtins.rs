use super::ast::{Token, ensure_symbol, ensure_int};
use super::core::{Block, DFunc};
use super::runtime::{eval, eval_expr};

// add the basic builtins to a block.
pub fn add_builtins(block: &mut Block) {
    block.locals.insert(String::from("+"), plus as DFunc);
    block.locals.insert(String::from("if"), if_expr as DFunc);
}

pub fn plus(block: &mut Block, args: &[Token]) -> Token {
    let left_op_token = eval(block, &args[0]);
    let left_op = ensure_int(block, left_op_token);
    let right_op_token = eval(block, &args[1]);
    let right_op = ensure_int(block, right_op_token);
    return Token::Integer(left_op + right_op);
}

pub fn if_expr(block: &mut Block, args:  &[Token]) -> Token {
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

use std::collections::HashMap;

pub fn eval(args: Vec<Vec<String>>) -> Vec<String> {
    let mut block = Block::new();
    eval_block(&mut block, &args)
}

fn eval_block(block: &mut Block, statements: &Vec<Vec<String>>) -> Vec<String> {
    let mut result = Vec::new();
    for statement in statements {
        result = eval_expr(block, statement)
    }
    return result;
}

pub fn eval_expr(block: &mut Block, statement: &[String]) -> Vec<String> {
    if let Some((func_name, args)) = statement.split_first() {
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
    locals: HashMap<String, fn(&[String]) -> Vec<String>>
}

impl Block {
    pub fn new() -> Block {
        let mut block = Block {
            locals: HashMap::new(),
        };
        block.locals.insert(String::from("+"), plus as fn(&[String]) -> Vec<String>);
        return block;
    }
}

fn plus(args: &[String]) -> Vec<String> {
    let left_op = args[0].parse::<i32>().unwrap();
    let right_op = args[1].parse::<i32>().unwrap();
    let mut return_value = Vec::new();
    return_value.push((left_op + right_op).to_string());
    return return_value;
}

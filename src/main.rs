use std::{env};
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    loop {
        let inp = read();
        let result = eval(inp);
        print(result);
    }
}

fn main2() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let mut globals = HashMap::new();
    globals.insert(String::from("+"), plus as fn(&Vec<String>) -> Vec<String>);
    eval2(globals, &mut args)
}

fn read() -> Vec<String> {
    std::io::stdout().write(b">>> ").unwrap();
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    let mut result = Vec::new();
    for token in input.split(" ") {
        result.push(String::from(token));
    }
    return result;
}

fn eval(args: Vec<String>) -> Vec<String> {
    for s in statements {
        compile(s);
        eval = "foo";
    }
    return args;
}

fn print(values: Vec<String>) {
    for v in values {
        println!("{}", v);
    }
}

fn eval2(globals: HashMap<String, fn(&Vec<String>) -> Vec<String>>, args: &mut Vec<String>) {
    let func_name = args.remove(0);
    match globals.get(&func_name) {
        Some(f) => {
            let res = f(&args);
            for r in res {
                println!("{}", r);
            }
        },
        None => {
            println!("func {} not found", func_name);
        }
    }
}

fn plus(args: &Vec<String>) -> Vec<String> {
    let left_op = args[0].parse::<i32>().unwrap();
    let right_op = args[0].parse::<i32>().unwrap();
    let mut return_value = Vec::new();
    return_value.push((left_op + right_op).to_string());
    return return_value;
}

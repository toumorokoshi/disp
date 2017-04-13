use super::ast::Token;
use gh_vm;

struct Block {
    scope: gh_vm::Scope,
    oplist: gh_vm::OpList
}

impl Block {
    pub fn new() -> Block {
        return Block {
            scope: gh_vm::Scope::new(),
            oplist: gh_vm::OpList::new(),
        }
    }
}

// compile a token into a set of VM opcodes.
// NOTE: this can also execute code due to the compile-time
// execution support.
pub fn compile(token: &Token) -> gh_vm::Function {
    let mut block = Block::new();
    let result_obj = gen_token(&mut block, token);
    return gh_vm::Function::VMFunction(vm::VMFunction {
        name: String::from("__main__"),
        argument_names: vec![],
        scope: block.scope,
        ops: block.ops,
        return_type: result_obj.typ
    });
}

pub fn gen_token(block: &mut Block, token: &Token) -> gh_vm::LocalObject {
    match token {
        &Token::Expression(ref tl) => eval_expr(block, tl),
        &Token::List(ref tl) => add_int(&mut block, 0),
        &Token::Symbol(ref s) => panic!("symbol found for non-expr"),
        &Token::BangSymbol(ref s) => panic!("bang symbol found for non-expr"),
        &Token::Integer(i) => add_int(&mut block, i),
        &Token::Boolean(b) => add_int(&mut block, if b {1} else {0})
    }
}

pub fn gen_expr(block: &mut Block, expr: &Vec<Token>) -> gh_vm::LocalObject {
    /* let mut func: Option<DFunc> = None;
    let Some((func_token, args)) = statement.split_first() {
        {
            let func_name = ensure_symbol(func_token)
        }
    } */
    add_int(&mut block, 10)
}

pub fn add_int(block: &mut Block, value: i64) -> gh_vm::LocalObject {
    let obj = block.scope.allocate_local(types::INT_TYPE.clone());
    block.oplist.push(Op::IntLoad{register: obj.index, constant: value});
    obj
}

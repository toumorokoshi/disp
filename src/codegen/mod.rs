use super::ast::Token;
use super::Block;
use ghvm;

mod core;

use core::CodeGenerator;


struct Context {
    scope: ghvm::Scope,
    oplist: ghvm::OpList
}

impl Context {
    pub fn new() -> Context {
        return Context {
            scope: ghvm::Scope::new(),
            oplist: ghvm::OpList::new(),
        }
    }
}

// compile a token into a set of VM opcodes.
// NOTE: this can also execute code due to the compile-time
// execution support.
pub fn compile(block: &mut Block, token: &Token) -> ghvm::Function {
    let mut Context = Context::new();
    let result_obj = gen_token(&mut Context, token);
    return ghvm::Function::VMFunction(ghvm::VMFunction {
        name: String::from("__main__"),
        argument_names: vec![],
        scope: Context.scope,
        ops: Context.oplist,
        return_typ: result_obj.typ
    });
}

fn gen_token(Context: &mut Context, token: &Token) -> ghvm::LocalObject {
    match token {
        &Token::Expression(ref tl) => gen_expr(Context, tl),
        &Token::List(ref tl) => add_int(Context, 0),
        &Token::Symbol(ref s) => panic!("symbol found for non-expr"),
        &Token::BangSymbol(ref s) => panic!("bang symbol found for non-expr"),
        &Token::Integer(i) => add_int(Context, i),
        &Token::Boolean(b) => add_int(Context, if b {1} else {0}),
        &Token::None => add_int(Context, 1)
    }
}

fn gen_expr(Context: &mut Context, expr: &Vec<Token>) -> ghvm::LocalObject {
    /* let mut func: Option<DFunc> = None;
    let Some((func_token, args)) = statement.split_first() {
        {
            let func_name = ensure_symbol(func_token)
        }
    } */
    add_int(Context, 10)
}

fn add_int(Context: &mut Context, value: i64) -> ghvm::LocalObject {
    let obj = Context.scope.allocate_local(ghvm::INT_TYPE.clone());
    Context.oplist.push(ghvm::Op::IntLoad{register: obj.index, constant: value});
    obj
}

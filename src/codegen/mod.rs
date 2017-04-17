mod core;

use ghvm;
use self::core::CodeGenerator;
use super::{Block, Token};

// compile a token into a set of VM opcodes.
// NOTE: this can also execute code due to the compile-time
// execution support.
pub fn compile(block: &mut Block, token: &Token) -> ghvm::Function {
    let mut code_generator = CodeGenerator::new();
    let mut builder = ghvm::FunctionBuilder::new();
    let result_obj = gen_token(&mut builder, token);
    builder.add_return(&result_obj);
    return builder.build();
}

fn gen_token(builder: &mut ghvm::FunctionBuilder, token: &Token) -> ghvm::BuildObject {
    match token {
        &Token::Expression(ref tl) => gen_expr(builder, tl),
        &Token::List(ref tl) => add_int(builder, 0),
        &Token::Symbol(ref s) => panic!("symbol found for non-expr"),
        &Token::BangSymbol(ref s) => panic!("bang symbol found for non-expr"),
        &Token::Integer(i) => add_int(builder, i),
        &Token::Boolean(b) => add_int(builder, if b {1} else {0}),
        &Token::None => add_int(builder, 1)
    }
}

fn gen_expr(builder: &mut ghvm::FunctionBuilder, expr: &Vec<Token>) -> ghvm::BuildObject {
    /* let mut func: Option<DFunc> = None;
    let Some((func_token, args)) = statement.split_first() {
        {
            let func_name = ensure_symbol(func_token)
        }
    } */
    add_int(builder, 10)
}

fn add_int(builder: &mut ghvm::FunctionBuilder, value: i64) -> ghvm::BuildObject {
    let obj = builder.allocate_local(ghvm::Type::Int);
    builder.ops.push(ghvm::Op::IntLoad{register: obj.register, constant: value});
    return obj;
}

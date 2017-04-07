#[derive(Clone)]
pub enum Token {
    List(Vec<Token>),
    // Dict(HashMap<Token, Token>)
    Symbol(Box<String>),
    Integer(i64)
}

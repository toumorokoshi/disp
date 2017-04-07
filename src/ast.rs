pub enum Token {
    List(Vec<Token>),
    // Dict(HashMap<Token, Token>)
    Symbol(String),
    Integer(i64)
}

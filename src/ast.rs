use std::fmt;

#[derive(Clone)]
pub enum Token {
    List(Vec<Token>),
    // Dict(HashMap<Token, Token>)
    Symbol(Box<String>),
    Integer(i64),
    Boolean(bool),
    None
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Token::List(ref tl) => {
                let mut res = Ok(());
                for t in tl {
                    res = write!(f, "{}", t)
                }
                return res;
            },
            &Token::Symbol(ref s) => write!(f, "{}", s),
            &Token::Integer(i) => write!(f, "{}", i),
            &Token::Boolean(b) => write!(f, "{}", b),
            &Token::None => write!(f, "None")
        }
    }
}

use std::fmt;
use super::core::Block;

#[derive(Clone)]
pub enum Token {
    List(Vec<Token>),
    Expression(Vec<Token>),
    // Dict(HashMap<Token, Token>)
    Symbol(Box<String>),
    BangSymbol(Box<String>),
    Integer(i64),
    Boolean(bool),
    None
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Token::List(ref tl) => {
                write!(f, "[");
                for t in tl {
                    write!(f, "{} ", t);
                }
                write!(f, "]")
            },
            &Token::Expression(ref tl) => {
                write!(f, "(");
                for t in tl {
                    write!(f, "{} ", t);
                }
                write!(f, ")")
            },
            &Token::BangSymbol(ref s) => write!(f, "{}!", s),
            &Token::Symbol(ref s) => write!(f, "{}", s),
            &Token::Integer(i) => write!(f, "{}", i),
            &Token::Boolean(b) => write!(f, "{}", b),
            &Token::None => write!(f, "None")
        }
    }
}


pub fn ensure_symbol<'a>(t: &'a Token) -> &'a str {
    if let &Token::Symbol(ref s) = t {
        return s;
    }
    panic!("string token expected.");
}

pub fn ensure_int(block: &mut Block, t: Token) -> i64 {
    if let Token::Integer(i) = t {
        return i;
    }
    panic!("int token expected.");
}

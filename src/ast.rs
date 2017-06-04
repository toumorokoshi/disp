use std::fmt;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Token {
    List(Vec<Token>),
    Expression(Vec<Token>),
    Dict(Box<Dict>),
    Symbol(Box<String>),
    BangSymbol(Box<String>),
    Integer(i64),
    Boolean(bool),
    None
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum HashableToken {
    Symbol(Box<String>),
    Integer(i64),
    Boolean(bool),
    None
}

impl HashableToken {
    pub fn as_token(&self) -> Token {
        match self {
            &HashableToken::Symbol(ref s) => Token::Symbol(s.clone()),
            &HashableToken::Integer(i) => Token::Integer(i),
            &HashableToken::Boolean(b) => Token::Boolean(b),
            &HashableToken::None => Token::None,
        }
    }
}

pub type Dict = HashMap<HashableToken, Token>;

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
            &Token::Dict(ref d) => {
                write!(f, "{{");
                for (key, value) in d.iter() {
                    write!(f, "{}: {}", key, value);
                }
                write!(f, "}}")
            }
            &Token::None => write!(f, "None"),
        }
    }
}


pub fn ensure_symbol<'a>(t: &'a Token) -> &'a str {
    if let &Token::Symbol(ref s) = t {
        return s;
    }
    panic!("string token expected.");
}

pub fn ensure_int(t: Token) -> i64 {
    if let Token::Integer(i) = t {
        return i;
    }
    panic!("int token expected.");
}


impl fmt::Display for HashableToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &HashableToken::Symbol(ref s) => write!(f, "{}", s),
            &HashableToken::Integer(i) => write!(f, "{}", i),
            &HashableToken::Boolean(b) => write!(f, "{}", b),
            &HashableToken::None => write!(f, "None")
        }
    }
}

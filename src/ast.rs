use std::fmt;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub enum Token {
    List(Vec<Token>),
    Expression(Vec<Token>),
    Map(Box<Map>),
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

pub type Map = HashMap<HashableToken, Token>;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Token::List(ref tl) => {
                try!(write!(f, "["));
                for t in tl {
                    try!(write!(f, "{} ", t));
                }
                write!(f, "]")
            },
            &Token::Expression(ref tl) => {
                try!(write!(f, "("));
                for t in tl {
                    try!(write!(f, "{} ", t));
                }
                write!(f, ")")
            },
            &Token::BangSymbol(ref s) => write!(f, "{}!", s),
            &Token::Symbol(ref s) => write!(f, "{}", s),
            &Token::Integer(i) => write!(f, "{}", i),
            &Token::Boolean(b) => write!(f, "{}", b),
            &Token::Map(ref d) => {
                try!(write!(f, "{{"));
                for (key, value) in d.iter() {
                    try!(write!(f, "{}: {}", key, value));
                }
                write!(f, "}}")
            }
            &Token::None => write!(f, "None"),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
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

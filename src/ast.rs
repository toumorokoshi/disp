use super::DispError;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub enum Token {
    BangSymbol(Box<String>),
    Boolean(bool),
    Bytes(Box<String>),
    Comment(Box<String>),
    Expression(Vec<Token>),
    Integer(i64),
    List(Vec<Token>),
    // blocks are used to represent
    // execution, while lists are literal
    // values
    Block(Vec<Token>),
    Map(Box<Map>),
    None,
    Symbol(Box<String>),
    String(Box<String>),
}

impl Token {
    pub fn to_hashable(&self) -> Result<HashableToken, DispError> {
        match self {
            &Token::Symbol(ref s) => Ok(HashableToken::Symbol(s.clone())),
            &Token::Integer(i) => Ok(HashableToken::Integer(i)),
            &Token::Boolean(b) => Ok(HashableToken::Boolean(b)),
            &Token::None => Ok(HashableToken::None),
            t => Err(DispError::new(&format!(
                "unable to convert token {} to hashable",
                t
            ))),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum HashableToken {
    Symbol(Box<String>),
    Integer(i64),
    Boolean(bool),
    None,
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
            }
            &Token::Block(ref tl) => {
                try!(write!(f, "<"));
                for t in tl {
                    try!(write!(f, "{} ", t));
                }
                write!(f, ">")
            }
            &Token::Expression(ref tl) => {
                try!(write!(f, "("));
                for t in tl {
                    try!(write!(f, "{} ", t));
                }
                write!(f, ")")
            }
            &Token::BangSymbol(ref s) => write!(f, "{}!", s),
            &Token::Bytes(ref b) => write!(f, "{}", b),
            &Token::Comment(ref s) => write!(f, "# {}", s),
            &Token::Symbol(ref s) => write!(f, "{}", s),
            &Token::String(ref s) => write!(f, "{}", s),
            &Token::Integer(i) => write!(f, "{}", i),
            &Token::Boolean(b) => write!(f, "{}", b),
            &Token::Map(ref d) => {
                try!(write!(f, "{{"));
                for (key, value) in d.iter() {
                    try!(write!(f, "{}: {},", key, value));
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
            &HashableToken::None => write!(f, "None"),
        }
    }
}

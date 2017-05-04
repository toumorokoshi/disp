/// contains all the parsing structures of ghvm
use super::{Token};

peg_file! grammar("grammar.rustpeg");

/// context for the parser
pub struct Context {}

pub fn parse(body: &str) -> Token {
    let processed_body = preprocess(body);
    let mut context = Context{};
    grammar::token(&processed_body, &mut context).unwrap()
}


/// preprocess a string, returning
/// back something that can be processed
/// by the peg parser
pub fn preprocess(body: &str) -> String {
    return String::from(body);
}

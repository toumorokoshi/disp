/// contains all the parsing structures of ghvm
use super::{Token};
use std::iter::Peekable;
peg_file! grammar("grammar.rustpeg");

#[cfg(test)]
mod tests;
mod preprocessor;

use self::preprocessor::preprocess;

pub fn parse(body: &str) -> Token {
    let processed_body = preprocess(body);
    grammar::token(&processed_body).unwrap()
}

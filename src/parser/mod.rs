/// contains all the parsing structures of ghvm
use pest::{
    Parser,
    iterators::Pair,
};
use super::{Token, Map};
use std::collections::HashMap;


#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct DispParser;

#[cfg(test)]
mod tests;
mod preprocessor;

use self::preprocessor::preprocess;

pub fn full_parse(body: &str) -> Token {
    let processed_body = preprocess(body);
    if cfg!(feature = "debug") {
        println!("DEBUG processed result: {}", &processed_body);
    }
    parse(&processed_body)
}

fn parse(body: &str) -> Token {
    let mut pairs = DispParser::parse(Rule::token, &body).unwrap_or_else(|e| panic!("{}", e));
    if let Some(pair) = pairs.next() {
        if cfg!(feature = "debug") {
            println!("DEBUG pest parser result: {:?}", pair.clone());
            println!("DEBUG pest string: {:?}", pair.clone().into_span().as_str());
        }
        return unpack(pair);
    }
    return Token::None;
}

/// Convert a token from the parser to a Disp token
fn unpack(pair: Pair<Rule>) -> Token {
    match pair.clone().as_rule() {
        _s @ Rule::bang_symbol => {
            // remove leading bang
            let string = pair.as_str().chars().skip(1).collect();
            Token::BangSymbol(Box::new(string))
        },
        _i @ Rule::integer => {
            Token::Integer(pair.as_str().parse::<i64>().unwrap())
        },
        _s @ Rule::symbol => {
            Token::Symbol(Box::new(String::from(pair.as_str())))
        },
        _n @ Rule::none => Token::None,
        _e @ Rule::expression => {
            let mut tokens = vec![];
            for p in pair.into_inner() {
                tokens.push(unpack(p));
            }
            Token::Expression(tokens)
        },
        _l @ Rule::list => {
            let mut tokens = vec![];
            for p in pair.into_inner() {
                tokens.push(unpack(p));
            }
            Token::List(tokens)
        },
        _m @ Rule::map => {
            Token::Map(Box::new(HashMap::new()))
        },
        _ => { Token::None }
    }
}

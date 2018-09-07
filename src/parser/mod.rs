/// contains all the parsing structures of ghvm
use super::{Token, HashableToken, Dict};
use pest::{
    Parser,
    iterators::Pair,
};


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
    let mut pairs = DispParser::parse(Rule::Token, &body).unwrap_or_else(|e| panic!("{}", e));
    if let Some(pair) = pairs.next() {
        if cfg!(feature = "debug") {
            println!("DEBUG pest parser result: {}", pair.clone().into_span().as_str());
        }
        return unpack(pair);
    }
    return Token::None;
}

/// Convert a token from the parser to a Disp token
fn unpack(pair: Pair<Rule>) -> Token {
    match pair.clone().as_rule() {
        s @ Rule::BangSymbol => {
            // remove leading bang
            let string = pair.as_str().chars().skip(1).collect();
            Token::BangSymbol(Box::new(string))
        },
        i @ Rule::Integer => {
            Token::Integer(pair.as_str().parse::<i64>().unwrap())
        },
        s @ Rule::Symbol => {
            Token::Symbol(Box::new(String::from(pair.as_str())))
        },
        n @ Rule::None => Token::None,
        e @ Rule::Expression => {
            let mut tokens = vec![];
            for p in pair.into_inner() {
                tokens.push(unpack(p));
            }
            Token::Expression(tokens)
        },
        l @ Rule::List => {
            let mut tokens = vec![];
            for p in pair.into_inner() {
                tokens.push(unpack(p));
            }
            Token::List(tokens)
        },
        _ => { Token::None }
    }
}

use super::Token;
/// contains all the parsing structures of ghvm
use pest::{iterators::Pair, Parser};
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "parser/grammar_indented.pest"]
struct DispParser;

#[cfg(test)]
mod tests;

pub fn parse(body: &str) -> Token {
    parse_rule(Rule::head, body)
}

fn parse_rule(rule: Rule, body: &str) -> Token {
    let mut pairs = DispParser::parse(rule, &body).unwrap_or_else(|e| panic!("{}", e));
    if let Some(pair) = pairs.next() {
        if cfg!(feature = "debug") {
            println!("DEBUG pest parser result: {:?}", pair.clone());
            println!("DEBUG pest string: {:?}", pair.clone().as_span().as_str());
        }
        return unpack(pair);
    }
    return Token::None;
}

/// Convert a token from the parser to a Disp token
fn unpack(pair: Pair<Rule>) -> Token {
    match pair.clone().as_rule() {
        _s @ Rule::bang_symbol => Token::BangSymbol(Box::new(String::from(pair.as_str()))),
        _c @ Rule::comment => Token::Comment(Box::new(String::from(pair.as_str()))),
        _e @ Rule::expression_no_parens => {
            let mut tokens = vec![];
            for p in pair.into_inner() {
                tokens.push(unpack(p));
            }
            Token::Expression(tokens)
        }
        _e @ Rule::expression => {
            let mut tokens = vec![];
            for p in pair.into_inner() {
                tokens.push(unpack(p));
            }
            Token::Expression(tokens)
        }
        _f @ Rule::false_value => Token::Boolean(false),
        _e @ Rule::integer => Token::Integer(pair.as_str().parse::<i64>().unwrap()),
        _l @ Rule::list_of_lines => {
            let mut tokens = vec![];
            for p in pair.into_inner() {
                tokens.push(unpack(p));
            }
            Token::List(tokens)
        }
        _l @ Rule::list => {
            let mut tokens = vec![];
            for p in pair.into_inner() {
                tokens.push(unpack(p));
            }
            Token::List(tokens)
        }
        _m @ Rule::map => {
            let mut map = HashMap::new();
            let mut pairs = pair.into_inner();
            let mut maybe_key = pairs.next();
            while let Some(key) = maybe_key {
                let value = pairs.next().expect("value not found corresponding to key");
                map.insert(unpack(key).to_hashable().unwrap(), unpack(value));
                maybe_key = pairs.next();
            }
            Token::Map(Box::new(map))
        }
        _s @ Rule::string => Token::String(Box::new(String::from(pair.as_str()))),
        _s @ Rule::symbol => Token::Symbol(Box::new(String::from(pair.as_str()))),
        _t @ Rule::true_value => Token::Boolean(true),
        _ => Token::None,
    }
}

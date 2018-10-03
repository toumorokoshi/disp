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
        }
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
        _m @ Rule::map => Token::Map(Box::new(HashMap::new())),
        _s @ Rule::string => Token::String(Box::new(String::from(pair.as_str()))),
        _s @ Rule::symbol => Token::Symbol(Box::new(String::from(pair.as_str()))),
        _t @ Rule::true_value => Token::Boolean(true),
        _ => Token::None,
    }
}

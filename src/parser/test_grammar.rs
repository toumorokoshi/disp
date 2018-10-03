use super::Token;
use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "parser/grammar_indented.pest"]
struct P;

fn parse(body: &str) -> Token {
    let mut pairs = P::parse(Rule::list, &body).unwrap_or_else(|e| panic!("{}", e));
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
        _l @ Rule::list => {
            let mut tokens = vec![];
            for p in pair.into_inner() {
                tokens.push(unpack(p));
            }
            Token::List(tokens)
        }
        _e @ Rule::expression => {
            let mut tokens = vec![];
            for p in pair.into_inner() {
                tokens.push(unpack(p));
            }
            Token::Expression(tokens)
        }
        _s @ Rule::symbol => Token::Symbol(Box::new(String::from(pair.as_str()))),
        _ => Token::None,
    }
}

#[test]
fn test_parser_string_expression() {
    assert_eq!(
        parse("print foo"),
        Token::List(vec![Token::Expression(vec![
            Token::Symbol(Box::new(String::from("print"))),
            Token::Symbol(Box::new(String::from("foo")))
        ])])
    );
}

#[test]
fn test_parser_multiple_expression() {
    assert_eq!(
        parse("print foo\nprint bar"),
        Token::List(vec![
            Token::Expression(vec![
                Token::Symbol(Box::new(String::from("print"))),
                Token::Symbol(Box::new(String::from("foo")))
            ]),
            Token::Expression(vec![
                Token::Symbol(Box::new(String::from("print"))),
                Token::Symbol(Box::new(String::from("bar")))
            ]),
        ])
    );
}

#[test]
fn test_parser_list() {
    assert_eq!(
        parse("print [foo print]"),
        Token::List(vec![Token::Expression(vec![
            Token::Symbol(Box::new(String::from("print"))),
            Token::List(vec![
                Token::Symbol(Box::new(String::from("foo"))),
                Token::Symbol(Box::new(String::from("print"))),
            ])
        ]),])
    );
}

use super::{parse, parse_rule, Rule, Token};
use std::collections::HashMap;

#[test]
fn test_parser_integer() {
    assert_eq!(parse_rule(Rule::token, "10"), Token::Integer(10));
}

#[test]
fn test_parser_symbol() {
    assert_eq!(
        parse_rule(Rule::token, "foobar"),
        Token::Symbol(Box::new(String::from("foobar")))
    );
    assert_eq!(
        parse_rule(Rule::token, "foobar-dash"),
        Token::Symbol(Box::new(String::from("foobar-dash")))
    );
}

#[test]
fn test_parser_bangsymbol() {
    assert_eq!(
        parse_rule(Rule::token, "!foobar"),
        Token::BangSymbol(Box::new(String::from("foobar")))
    );
}

#[test]
fn test_parser_none() {
    assert_eq!(parse_rule(Rule::token, "None"), Token::None);
}

#[test]
fn test_parser_list() {
    assert_eq!(
        parse_rule(Rule::token, "[ 1 10 ]"),
        Token::List(vec![Token::Integer(1), Token::Integer(10)])
    );
}

#[test]
fn test_parser_empty_map() {
    assert_eq!(
        parse_rule(Rule::token, "{}"),
        Token::Map(Box::new(HashMap::new()))
    );
}

#[test]
fn test_parser_string() {
    assert_eq!(
        parse_rule(Rule::token, "\"foo\""),
        Token::String(Box::new(String::from("foo")))
    );
}

#[test]
fn test_parser_string_expression() {
    assert_eq!(
        parse_rule(Rule::token, "(print-string \"foo\")"),
        Token::Expression(vec![
            Token::Symbol(Box::new(String::from("print-string"))),
            Token::String(Box::new(String::from("foo")))
        ])
    );
}

#[test]
fn test_parser_expression_no_parens() {
    assert_eq!(
        parse_rule(Rule::expression_no_parens, "print foo"),
        Token::Expression(vec![
            Token::Symbol(Box::new(String::from("print"))),
            Token::Symbol(Box::new(String::from("foo")))
        ])
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
fn test_parser_list_in_expression() {
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

#[test]
fn test_parser_indented_list() {
    assert_eq!(
        parse("print\n\tprint\nbar"),
        Token::List(vec![
            Token::Expression(vec![
                Token::Symbol(Box::new(String::from("print"))),
                Token::List(vec![Token::Expression(vec![Token::Symbol(Box::new(
                    String::from("print")
                )),]),]),
            ]),
            Token::Expression(vec![Token::Symbol(Box::new(String::from("bar"))),]),
        ]),
    );
}

#[test]
fn test_parser_map() {
    let mut m = HashMap::new();
    m.insert(
        Token::Boolean(true).to_hashable().unwrap(),
        Token::Integer(1),
    );
    m.insert(
        Token::Boolean(false).to_hashable().unwrap(),
        Token::Integer(1),
    );
    assert_eq!(
        parse("{true: 1, false: 1,}"),
        Token::List(vec![Token::Expression(vec![Token::Map(Box::new(m))]),]),
    );
}

#[test]
fn test_parser_multiline_map() {
    let mut m = HashMap::new();
    m.insert(
        Token::Boolean(true).to_hashable().unwrap(),
        Token::Integer(1),
    );
    m.insert(
        Token::Boolean(false).to_hashable().unwrap(),
        Token::Integer(2),
    );
    assert_eq!(
        parse(
            "{
\ttrue: 1,
\tfalse: 2,
}"
        ),
        Token::List(vec![Token::Expression(vec![Token::Map(Box::new(m))]),]),
    );
}

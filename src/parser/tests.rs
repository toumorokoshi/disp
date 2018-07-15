use super::{preprocess, parse, Token};

#[test]
fn test_preprocess() {
    let text = "if (== 1 0)";
    let result = preprocess(text);
    assert_eq!(result, "[(if (== 1 0)) ]");
}

#[test]
fn test_preprocess_2() {
    let ident_text = r#"if (== i 0)
	print hello"#;
    let result = preprocess(ident_text);
    println!("{}", result);
    assert_eq!(result, "[(if (== i 0) [(print hello) ]) ]");
}


#[test]
fn test_preprocess_empty_line() {
    let ident_text = r#"print 1
"#;
    let result = preprocess(ident_text);
    println!("{}", result);
    assert_eq!(result, "[(print 1) ]");
}


#[test]
fn test_remove_comments() {
    let ident_text = r#"print 1
# this is a comment {}[]()
"#;
    let result = preprocess(ident_text);
    println!("{}", result);
    assert_eq!(result, "[(print 1) ]");
}


#[test]
fn test_preprocess_parens() {
    let ident_text = r#"+ 1 1
+ 1 2"#;
    let result = preprocess(ident_text);
    println!("{}", result);
    assert_eq!(result, "[(+ 1 1) (+ 1 2) ]")
}

/// dictionaries should be preprocessed into a single line
#[test]
fn test_preprocess_dict() {
    let indent_text = r#"match 1 {
    1: (print 2)
}"#;
    let result = preprocess(indent_text);
    println!("{}", result);
    assert_eq!(result, "[(match 1 {    1: (print 2)}) ]");
}


fn test_preprocess_list() {
    let ident_text = "[(+ 1 1)]";
    let result = preprocess(ident_text);
    println!("{}", result);
    assert_eq!(result, "[(+ 1 1)]");
}


#[test]
fn test_parser_integer() {
    let ident_text = "10";
    let result = parse(ident_text);
    assert_eq!(result, Token::Integer(10));
}

#[test]
fn test_parser_str() {
    let ident_text = "foobar";
    let result = parse(ident_text);
    assert_eq!(result, Token::Symbol(Box::new(String::from("foobar"))));
}

#[test]
fn test_parser_bangsymbol() {
    let ident_text = "!foobar";
    let result = parse(ident_text);
    assert_eq!(result, Token::BangSymbol(Box::new(String::from("foobar"))));
}


#[test]
fn test_parser_none() {
    let ident_text = "None";
    let result = parse(ident_text);
    assert_eq!(result, Token::None);
}


#[test]
fn test_parser_list() {
    let ident_text = "[ 1 10 ]";
    let result = parse(ident_text);
    assert_eq!(result, Token::List(vec![
        Token::Integer(1),
        Token::Integer(10),
    ]));
}

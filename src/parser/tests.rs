use super::preprocess;

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


// TODO: support this case
fn test_preprocess_list() {
    let ident_text = "[(+ 1 1)]";
    let result = preprocess(ident_text);
    println!("{}", result);
    assert_eq!(result, "[(+ 1 1)]");
}

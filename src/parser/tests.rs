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
fn test_preprocess_parens() {
    let ident_text = r#"+ 1 1
+ 1 2"#;
    let result = preprocess(ident_text);
    println!("{}", result);
    assert_eq!(result, "[(+ 1 1) (+ 1 2) ]")
}

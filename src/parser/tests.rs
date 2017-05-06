use super::preprocess;

#[test]
fn test_preprocess() {
    let ident_text = r#"if (== i 0)
	print hello"#;
    let result = preprocess(ident_text);
    println!("{}", result);
    assert_eq!(result, "[(if (== i 0) [(print hello)])]");
}


fn test_preprocess_parens() {
    let ident_text = r#"+ 1 1
+ 1 2"#;
    let result = preprocess(ident_text);
    println!("{}", result);
    assert_eq!(result, "[(+ 1 1) (+ 1 2)]")
}

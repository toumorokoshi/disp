use super::preprocess;

#[test]
fn test_preprocess() {
    let ident_text = r#"if (== i 0)
	(print hello)
"#;
    let result = preprocess(ident_text);
    println!("{}", result);
    assert_eq!(result, "if (== i 0) [(print hello) ]");
}

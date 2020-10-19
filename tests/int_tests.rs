use string_util::Capitalization;

#[test]
fn capitalization() {
    let result = string_util::capitalize(String::from("ngozi"), Capitalization::UpperCase).unwrap();
    let expected = String::from("Ngozi");
    assert_eq!(result, expected);

    let result =
        string_util::capitalize(String::from("Shalewa"), Capitalization::LowerCase).unwrap();
    let expected = String::from("shalewa");
    assert_eq!(result, expected);

    let result = string_util::capitalize(String::from(" "), Capitalization::LowerCase);
    result.expect_err("The string is not empty");
}

#[test]
fn vector_to_string() {
    let vect1 = vec!["APC".to_string(), "PDP".to_string(), "ANPP".to_string()];
    let vect2 = vec![
        "127".to_string(),
        "0".to_string(),
        "0".to_string(),
        "1".to_string(),
    ];

    let delim_string_1 = string_util::vec_to_delimited_str(&vect1, ':');
    let delim_string_2 = string_util::vec_to_delimited_str(&vect2, '.');

    assert!(delim_string_2.contains("127.0.0.1"));
    assert!(delim_string_1.contains("APC:PDP:ANPP"));
}

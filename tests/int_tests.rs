use string_util::{Capitalization};

#[test]
fn email_check() {
    
    let valid1 = "ade.32@rust.com".to_owned();
    let valid2 = "ade@rust.com".to_owned();

    // Write assertions for these.
    let valid3 = "ade-65@rust.com".to_owned();
    let valid4 = "ade_65@rust.com".to_owned();

    let valid5 = "ade_65@".to_owned();
    let invalid1 = "ade@com".to_owned();
    let invalid2 = "ade@rust..com".to_owned();
    //let invalid3 = "ade@com".to_owned();
    let invalid4 = "ade@.com".to_owned();

    // todo: write assertions for thee
    let invalid5 = "ade..34@rust.com".to_owned();
    let invalid6 = "ade#34@rust.com".to_owned();
    let invalid7 = "ade__34@rust.com".to_owned();
    let invalid8 = "ade--34@rust.com".to_owned();
    let invalid9 = "ade.@rust.com".to_owned();
    let invalid10 = "ade..34@rust.com".to_owned();
    let invalid11 = ".ade@rust.com".to_owned();


    assert!(!string_util::is_valid_email(&invalid4));
    //assert!(!string_util::is_valid_email(&invalid3));
    assert!(!string_util::is_valid_email(&invalid2));
    assert!(!string_util::is_valid_email(&invalid1));
    assert!(!string_util::is_valid_email(&valid1));
    assert!(!string_util::is_valid_email(&valid2));
}

#[test]
fn capitalization() {

    let result = string_util::capitalize(String::from("ngozi"), Capitalization::UpperCase).unwrap();
    let expected = String::from("Ngozi");
    assert_eq!(result, expected);
    
    let result = string_util::capitalize(String::from("Shalewa"), Capitalization::LowerCase).unwrap();
    let expected = String::from("shalewa");
    assert_eq!(result, expected);
}

#[test]
fn vector_to_string() {

    let str_vector = vec!["APC".to_string(), "PDP".to_string(), "ANPP".to_string()];
    let empty_vec: Vec<String> = Vec::new();

    let delimited_string = string_util::vec_to_delimited_str(&str_vector, ',');
    let empty_string = string_util::vec_to_delimited_str(&empty_vec, ',');

    assert!(empty_string.is_empty());
    assert!(delimited_string.contains("APC,PDP,ANPP"));
}

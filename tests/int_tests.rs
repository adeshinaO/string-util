// Cargo compiles every Rust file in `/tests` as a separate crate.
// This 'use' statement is needed to bring 
use string_util;

#[test]
fn email_check() {
    // Todo: rename this method.
}

#[test]
fn capitalize() {
    // Todo: test decapitalization here too.
}

#[test]
fn string_vector_conversion() {

    let delimited_string = "Ondo,Edo,Musa,Tayo".to_string();
    let empty_string = "".to_string();

    let vector = string_util::delimited_str_to_vec(&delimited_string);
    let empty_vector = string_util::delimited_str_to_vec(&empty_string);

    assert!(vector.len() == 4);
    assert_eq!(vector.get(0), Some(&String::from("Ondo")));
    assert!(empty_vector.is_empty());

    let str_vector = vec!["APC".to_string(), "PDP".to_string()];
    let empty_vec: Vec<String> = Vec::new();

    delimited_string = string_util::vec_to_delimited_str(&str_vector);
    empty_string = string_util::vec_to_delimited_str(&empty_vec);

    assert!(empty_string.is_empty());
    assert!(delimited_string.contains("APC,PDP"));
}

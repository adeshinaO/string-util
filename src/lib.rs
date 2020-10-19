//! # `string_util` provides support for a small number of operations involving strings.

use std::fmt::{Display, Formatter, Result as FmtResult};

/// Capitalizes the first character of the given string according to `cap_type`.
/// Returns `Err(EmptyStringError)` if the string is empty.
///
/// # Example
///
/// ```
/// use string_util::Capitalization;
///
/// let result = string_util::capitalize(String::from("ngozi"), Capitalization::UpperCase).unwrap();
/// assert_eq!(result,  String::from("Ngozi"));
///
/// let result = string_util::capitalize(String::from("Shalewa"), Capitalization::LowerCase).unwrap();
/// assert_eq!(result, String::from("shalewa"));
/// ```
pub fn capitalize(txt: String, cap_type: Capitalization) -> Result<String, EmptyStringError> {
    let txt = txt.trim();
    let mut char_vec = txt.chars().collect::<Vec<char>>();

    if let Some(first_char) = char_vec.get_mut(0) {
        if first_char.is_whitespace() {
            return Err(EmptyStringError);
        }

        if let Capitalization::UpperCase = cap_type {
            first_char.make_ascii_uppercase();
        } else {
            first_char.make_ascii_lowercase();
        }

        let mut result = String::new();
        for ch in char_vec {
            let mut buf = [0; 4];
            let str_char = ch.encode_utf8(&mut buf);
            result.push_str(str_char);
        }
        Ok(result)
    } else {
        Err(EmptyStringError)
    }
}

/// Converts a vector of any `Display` type to a string delimited by the passed character.
///
/// # Example
///
/// ```
/// use string_util;
///
/// let vect1 = vec!["APC".to_string(), "PDP".to_string(), "ANPP".to_string()];
/// let vect2 = vec!["127".to_string(), "0".to_string(), "0".to_string(), "1".to_string()];
///
/// let delim_string_1 = string_util::vec_to_delimited_str(&vect1, ':');
/// let delim_string_2 = string_util::vec_to_delimited_str(&vect2, '.');
///
/// assert!(delim_string_2.contains("127.0.0.1"));
/// assert!(delim_string_1.contains("APC:PDP:ANPP"));
/// ```
pub fn vec_to_delimited_str<T: Display>(str_vec: &Vec<T>, delimiter: char) -> String {
    let mut result = String::new();

    for (i, str) in str_vec.iter().enumerate() {
        if i > 0 {
            result.push(delimiter);
        }
        result.push_str(str.to_string().as_str())
    }

    result
}
#[derive(Debug, Clone)]
pub struct EmptyStringError;

impl Display for EmptyStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "The supplied string is empty")
    }
}
pub enum Capitalization {
    LowerCase,
    UpperCase,
}

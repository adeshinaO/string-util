

// TODO: Write a custom error type (ErrorKind ?) to replace 'String'... see rust by example

// TODO: Skim the book chapter on Strings

// Read Rust guide on API design guide.

use std::fmt::{Formatter, Display, Result as FmtResult};
use regex::Regex;

// TODO: Don't just copy this. Read a short regex refresher tutorial and write the regex.
// r""" is used for raw string. escape characters are not processed.
const EMAIL_REGEX_PATTERN: &str = r"";

#[derive(Debug, Clone)]
pub struct EmptyStringError;


impl Display for EmptyStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "The supplied string is empty")
    }
}

pub fn is_email(txt: &String) -> bool {

    if !txt.is_empty() {
        // Panic is acceptable since the Regex should never be wrong.
        let email_pattern = Regex::new(EMAIL_REGEX_PATTERN).unwrap(); 
        return email_pattern.is_match(txt);
    }

    false 
}

pub fn capitalize(txt: &mut String) -> Result<(), EmptyStringError> {

    // capitalize txt

    // if txt is empty, send EmptyStringError

    Ok(())
}

pub fn decapitize(txt: &mut String) -> Result<(), EmptyStringError> {

    

   
    Ok(())
}

pub fn delimited_str_to_vec(txt: &String) -> Vec<String> {

    // Convert a comma delimited string to a Vec<String>


    // If the string is empty, return an empty vector.

    vec!["K".to_owned()]

} 

pub fn vec_to_delimited_str(vec: &Vec<String>) -> String {
    
    // If the vector is empty, return an empty string.

    Err("".to_owned())

}



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

pub fn is_valid_email(txt: &String) -> bool {

    // IF this crate is going to be published, then this method has to be thorough
    // Use wikipedia to figure out what to do.

    // This won't work!
    /*if !txt.is_empty() {
        // Panic is acceptable since the Regex should never be wrong.
        let email_pattern = Regex::new(EMAIL_REGEX_PATTERN).unwrap(); 
        return email_pattern.is_match(txt);
    } */

    // TODO:
    // Check if the email contains '@' and use that to split the address into username and domain.
    // Validate username and domain separately, if both are valid then the address is valid


    // See link for what valid email.
    // https://en.wikipedia.org/wiki/Email_address


    let parts: Vec<&str> = txt.split('@').collect();

    if parts.len() != 2 {
        return false;
    }

    let is_valid_username = validate_email_username(parts.get(0).unwrap());
    let is_valid_domain = validate_email_domain(parts.get(1).unwrap());

  
    is_valid_domain && is_valid_username
}

pub fn capitalize(txt: String, cap_type: Capitalization) -> Result<String, EmptyStringError> {

    let mut char_vec = txt.chars().collect::<Vec<char>>();

    if let Some(first_char) = char_vec.get_mut(0) {
        
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


pub fn vec_to_delimited_str<T: Display>(str_vec: &Vec<T>, delimiter: char) -> String {
    
    // If the vector is empty, return an empty string.
    let mut result = String::new();

    for (i, str) in str_vec.iter().enumerate() {
        if i > 0 {
            result.push(delimiter);
        }
        result.push_str(str.to_string().as_str())
    }

    result
}

pub enum Capitalization {
    LowerCase,
    UpperCase
}

fn validate_email_domain(domain: &str) -> bool {


    false
}

fn validate_email_username(username: &str) -> bool {


    false
}


#[cfg(test)]
mod tests {

    #[test]
    fn checK_username_validation() {
    
    }

    fn check_domain_validation() {

    }
}

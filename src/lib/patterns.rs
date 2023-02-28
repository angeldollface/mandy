/*
MANDY-C by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

use regex::Regex;
use std::collections::HashMap;

pub fn patterns() -> HashMap<String, Regex> {
    let mut result: HashMap<String, Regex> = HashMap::new();
    result.insert(String::from("EXPORT"), Regex::new(r"(export)").unwrap());
    result.insert(String::from("FUNCTION_KEYWORD"), Regex::new(r"(funky)").unwrap());
    result.insert(String::from("OPEN_PAREN"), Regex::new(r"(\()").unwrap());
    result.insert(String::from("CLOSE_PAREN"), Regex::new(r"(\))").unwrap());
    result.insert(String::from("OPEN_CURLY"), Regex::new(r"(\{)").unwrap());
    result.insert(String::from("CLOSE_CURLY"), Regex::new(r"(\})").unwrap());
    result.insert(String::from("ENTITY"), Regex::new(r"(\w+)").unwrap());
    result.insert(String::from("TYPE_DECLARATION"), Regex::new(r"(:)").unwrap());
    result.insert(String::from("RETURN_TYPE"), Regex::new(r"(->)").unwrap());
    result.insert(String::from("ASSIGN_KEYWORD"), Regex::new(r"(=)").unwrap());
    result.insert(String::from("ADD"), Regex::new(r"(\+)").unwrap());
    result.insert(String::from("DIV"), Regex::new(r"(/)").unwrap());
    result.insert(String::from("MUL"), Regex::new(r"(\*)").unwrap());
    result.insert(String::from("MINUS"), Regex::new(r"(\-)").unwrap());
    result.insert(String::from("STRING"), Regex::new(r"'(.*)'").unwrap());
    result.insert(String::from("RETURN_KEYWORD"), Regex::new(r"(trip)").unwrap());
    result.insert(String::from("EOL"), Regex::new(r"(;)").unwrap());
    return result;
}
/*
MANDY-C by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

use regex::Regex;
use super::utils::clean_split;
use std::collections::HashMap;
use serde_json::to_string_pretty;

#[derive(Clone, PartialEq)]
pub struct Token {
    pub name: String,
    pub value: String
}

impl Token {
    pub fn to_string(&self) -> String {
        return format!("{} => {}", &self.name, &self.value);
    }
    pub fn to_map(self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();
        result.insert(self.name, self.value);
        return result;
    }
}

#[derive(Clone, PartialEq)]
pub struct TokenStream {
    pub tokens: Vec<Token>
}

impl TokenStream {
    pub fn to_json(&self) -> String {
        let mut json_src_string_vec: Vec<HashMap<String, String>> = Vec::new();
        for token in self.tokens.clone(){
            json_src_string_vec.push(token.to_map());
        }
        let result: String = to_string_pretty(&json_src_string_vec).unwrap();
        return result;
    }
}

/// The actual lexing function: Iterates through all lines
/// and then through all characters and builds a vector of tokens
/// while doing so and finally returns this vector.
pub fn lex(source_code: &String, pattern_pool: &HashMap<String, Regex>) -> Vec<Token>{
    let lines: Vec<String> = clean_split(source_code, &String::from("\n"));
    let mut result: Vec<Token> = Vec::new();
    for line in lines {
        let char_list: Vec<String> = clean_split(&line, &String::from(""));
        let mut new_char_list: Vec<String> = Vec::new();
        for char_item in char_list {
            new_char_list.push(char_item);
            let collected_chars: String = new_char_list.join("");
            for (key,value) in pattern_pool.into_iter() {
                if value.is_match(&collected_chars) {
                    new_char_list.clear();
                    let captured = value.captures(&collected_chars).unwrap();
                    let new_token: Token = Token {
                        name: key.to_string(),
                        value: captured.get(1).unwrap().as_str().to_string()
                    };
                    result.push(new_token);
                }
                else {}
            }
        }
    }
    return result;
}
/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

use std::fs::read_to_string;

/// Returns a vector of strings from a character split for a string.
/// Both the string and split character have to be strings.
pub fn clean_split(subject: &String, split_char: &String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in subject.split(split_char) {
        let new_item: String = item.to_string();
        result.push(new_item);
    }
    return result;
}

// Checks whether a file exists and
/// returns a boolean to that effect.
pub fn file_is(filename: &String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let contents = read_to_string(filename);
    match contents {
        Ok(_n) => result.push(true),
        Err(_x) => result.push(false)
    }
    return result[0];
}

/// Tries to read a file and return
/// its contents.
pub fn read_file(file_name: &String) -> String {
    let mut result: String = String::from("");
    if file_is(file_name) == true {
        result = read_to_string(file_name).unwrap();
    }
    else {}
    return result;
}
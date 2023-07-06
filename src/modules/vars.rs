/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing Rust's standard
/// "HashMap" API.
use std::collections::HashMap;

/// A function to set Mandy's executable variables.
pub fn mandy_vars() -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    result.insert(String::from("version"), String::from("0.2.0"));
    result.insert(String::from("name"), String::from("Mandy"));
    result.insert(String::from("author"), String::from("Angel Dollface"));
    result.insert(String::from("dist_folder"), String::from("dist"));
    result.insert(String::from("partials_dir"), String::from("partials"));
    result.insert(String::from("server_file"), String::from("server.ts"));
    result.insert(String::from("server_mod_url"), String::from("https://angeldollface.art/mandys-house/helpers/server.ts"));
    return result;
}
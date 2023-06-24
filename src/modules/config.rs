/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the function
/// to parse JSON into
/// Rust data structures
/// from the "serde_json"
/// crate.
use serde_json::from_str;

/// Importing Rust's standard
/// "HashMap" API.
use std::collections::HashMap;

/// Importing Mandy's error
/// struct.
use super::errors::MandyError;

/// A function to parse the configuration file in JSON
/// format of a Mandy site. Returns a result of a "HashMap"
/// or an error.
pub fn deserialize_config(json_string: &String) -> Result<
    HashMap<String, String>, MandyError
> {
    let result: Result<HashMap<String, String>, serde_json::Error> = from_str(
        json_string
    );
    match result {
        Ok(map) => {
            return Ok(map);
        },
        Err(e) => {
            return Err::<HashMap<String,String>, MandyError>(
                MandyError::new(&e.to_string())
            );
        }
    }
}
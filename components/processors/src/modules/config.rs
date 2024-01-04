/*
MANDY PROCESSORS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "toml"
/// crate to deserialize it.
use toml;

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// Importing the function
/// to parse JSON into
/// Rust data structures
/// from the "serde_json"
/// crate.
use serde_json::from_str;

/// Importing Rust's standard
/// "HashMap" API.
use std::collections::HashMap;

/// A function to parse the configuration file in JSON
/// format of a Mandy site. Returns a result of a "HashMap"
/// or an error.
pub fn deserialize_config_json(
    json_string: &String
) -> Result<HashMap<String, String>, MandyError> {
    let result: HashMap<String, String> = match from_str(json_string){
        Ok(result) => result,
        Err(e) => {
            return Err::<HashMap<String,String>, MandyError>(
                MandyError::new(&e.to_string())
            );
        }
    };
    Ok(result)
}

/// A function to parse the configuration file in TOML
/// format of a Mandy site. Returns a result of a "HashMap"
/// or an error.
pub fn deserialize_config_toml(
    toml_string: &String
) -> Result<HashMap<String, String>, MandyError> {
    let result: HashMap<String, String> = match toml::from_str(toml_string) {
        Ok(result) => result,
        Err(e) => {
            return Err::<HashMap<String,String>, MandyError>(
                MandyError::new(&e.to_string())
            );
        }
    };
    Ok(result)
}
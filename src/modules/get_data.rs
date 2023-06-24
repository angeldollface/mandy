/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the method from
/// the "coutils" crate to check
/// whether a file exists.
use coutils::dir_is;

/// Importing Rust's standard
/// "HashMap" API.
use std::collections::HashMap;

/// Importing Mandy's error
/// struct.
use super::errors::MandyError;

/// Importing the method to find
/// and detect JSON data files in
/// a Mandy site's "data" directory.
use super::data::find_data_files;

/// Importing the method to 
/// deserialize JSON files into
/// data structures Mandy's compiler
/// can use.
use super::data::deserialize_data;

/// Attempts to retrieve the data objects of a 
/// Mandy site if they exist.
pub fn get_data(
    dir: &String
) -> Result<Option<HashMap<String, HashMap<String, Vec<HashMap<String, String>>>>>,MandyError> {
    let data_dir_path: String = format!("{}/data", dir);
    if dir_is(&data_dir_path) {
        let data_strings: Vec<HashMap<String, String>> = find_data_files(&data_dir_path);
        let mut data = match deserialize_data(data_strings) {
            Ok(data) => data,
            Err(e) => {
                return Err::<Option<HashMap<String, HashMap<String, Vec<HashMap<String, String>>>>>, MandyError>(
                    MandyError::new(
                        &e.to_string()
                    )
                );
            }
        };
        return Ok(Some(data));
    }
    else {return Ok(None);}
}
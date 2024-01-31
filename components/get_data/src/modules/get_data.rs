/*
MANDY DATA by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the method from
/// the "coutils" crate to check
/// whether a directory exists.
use coutils::dir_is;

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// Importing Rust's standard
/// "HashMap" API.
use std::collections::HashMap;

/// Importing the method to find
/// and detect JSON data files in
/// a Mandy site's "data" directory.
use processors::find_data_files;

/// Importing the method to 
/// deserialize JSON files into
/// data structures Mandy's compiler
/// can use.
use processors::deserialize_data_json;

/// Importing the method to 
/// deserialize YAML files into
/// data structures Mandy's compiler
/// can use.
use processors::deserialize_data_yaml;

/// Importing the method to 
/// deserialize TOML files into
/// data structures Mandy's compiler
/// can use.
use processors::deserialize_data_toml;

/// Attempts to retrieve the data objects of a 
/// Mandy site if they exist.
pub fn get_data(
    dir: &String
) -> Result<Option<HashMap<String, Vec<HashMap<String, String>>>>,MandyError> {
    let data_dir_path: String = format!("{}/data", dir);
    if dir_is(&data_dir_path) {
        let data_strings: Vec<HashMap<String, String>> = match find_data_files(&data_dir_path){
            Ok(data_strings) => data_strings,
            Err(e) => {
                return Err::<Option<HashMap<String, Vec<HashMap<String, String>>>>, MandyError>(
                    MandyError::new(&e.to_string())
                );
            }
        };
        if data_strings.is_empty(){
            let err_msg: &String = &format!("\"{}/data\" is empty!", dir);
            return Err::<Option<HashMap<String, Vec<HashMap<String, String>>>>, MandyError>(
                MandyError::new(&&err_msg.to_string())
            );
        }
        else {
            match data_strings.data_file_type{
                JSON => {
                    let data = match deserialize_data_json(data_strings) {
                        Ok(data) => data,
                        Err(e) => {
                            return Err::<Option<HashMap<String, Vec<HashMap<String, String>>>>, MandyError>(
                                MandyError::new(
                                    &e.to_string()
                                )
                            );
                        }
                    };
                    return Ok(Some(data));
                },
                YAML => {
                    let data = match deserialize_data_yaml(data_strings) {
                        Ok(data) => data,
                        Err(e) => {
                            return Err::<Option<HashMap<String, Vec<HashMap<String, String>>>>, MandyError>(
                                MandyError::new(
                                    &e.to_string()
                                )
                            );
                        }
                    };
                    return Ok(Some(data));
                },
                TOML => {
                    let data = match deserialize_data_toml(data_strings) {
                        Ok(data) => data,
                        Err(e) => {
                            return Err::<Option<HashMap<String, Vec<HashMap<String, String>>>>, MandyError>(
                                MandyError::new(
                                    &e.to_string()
                                )
                            );
                        }
                    };
                    return Ok(Some(data));
                },
                _ => {
                    let e: String = String::from("An error occurred with parsing data files!");
                    return Err::<Option<HashMap<String, Vec<HashMap<String, String>>>>, MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );

                }
            };
            
        }
    }
    else {return Ok(None);}
}
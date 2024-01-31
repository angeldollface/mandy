/*
MANDY PROCESSORS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

use coutils::file_type;
/// Importing the "toml"
/// crate to deserialize TOML.
use toml;

/// Importing the "serde_yaml"
/// crate to deserialize YAML.
use serde_yaml;

/// Importing the "Entity"
/// enum from the "coutils"
/// crate to determine the type
/// of file.
use coutils::Entity;

/// Importing the "read_file"
/// method from the "coutils"
/// crate to read text files.
use coutils::read_file;

/// Importing the "FileEntry"
/// struct to work with files easier.
use coutils::FileEntry;

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// Importing the "from_str"
/// method from the "serde_json"
/// crate to store JSON
/// into Rust data structures.
use serde_json::from_str;

/// Importing the "clean_split"
/// method from the "coutils"
/// crate to split strings.
use coutils::clean_split;

/// Importing Rust's standard
/// "HashMap" data structure.
use std::collections::HashMap;

/// Importing the method to store
/// information about a directory's
/// contents.
use coutils::list_dir_contents;

/// Deserializing JSON data from the "data"
/// directory and further processing this.
pub fn deserialize_data_json(
    data_strings: Vec<HashMap<String, String>>
) -> Result<
HashMap<String, Vec<HashMap<String, String>>>, 
MandyError
> {
    let mut result: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    for item in data_strings.into_iter() {
        for (k,v) in item.into_iter() {
            let file_name: &String = &k;
            let map: Vec<HashMap<String, String>> = match from_str(&v) {
                Ok(map) => map,
                Err(e) => {
                    let msg: String = format!("Error in file \"{}.json\":\n{}", file_name, e);
                    return Err::<HashMap<String, Vec<HashMap<String, String>>>, MandyError>(MandyError::new(&msg.to_string()));
                }
            };
            result.insert(file_name.to_owned(), map);
        }
    }
    Ok(result)
}

/// Deserializing YAML data from the "data"
/// directory and further processing this.
pub fn deserialize_data_yaml(
    data_strings: Vec<HashMap<String, String>>
) -> Result<
HashMap<String, Vec<HashMap<String, String>>>, 
MandyError
> {
    let mut result: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    for item in data_strings.into_iter() {
        for (k,v) in item.into_iter() {
            let file_name: &String = &k;
            let map: Vec<HashMap<String, String>> = match serde_yaml::from_str(&v) {
                Ok(map) => map,
                Err(e) => {
                    let msg: String = format!("Error in file \"{}.yaml\":\n{}", file_name, e);
                    return Err::<HashMap<String, Vec<HashMap<String, String>>>, MandyError>(MandyError::new(&msg.to_string()));
                }
            };
            result.insert(file_name.to_owned(), map);
        }
    }
    Ok(result)
}

/// Deserializing YAML data from the "data"
/// directory and further processing this.
pub fn deserialize_data_toml(
    data_strings: Vec<HashMap<String, String>>
) -> Result<
HashMap<String, Vec<HashMap<String, String>>>, 
MandyError
> {
    let mut result: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    for item in data_strings.into_iter() {
        for (k,v) in item.into_iter() {
            let file_name: &String = &k;
            let map: Vec<HashMap<String, String>> = match toml::from_str(&v) {
                Ok(map) => map,
                Err(e) => {
                    let msg: String = format!("Error in file \"{}.toml\":\n{}", file_name, e);
                    return Err::<HashMap<String, Vec<HashMap<String, String>>>, MandyError>(MandyError::new(&msg.to_string()));
                }
            };
            result.insert(file_name.to_owned(), map);
        }
    }
    return Ok(result);
}

/// Storing the file contents
/// from data files into
/// a list of maps.
pub fn find_data_files(
    dir: &String, ending: &str
) -> Result<Vec<HashMap<String, String>>, MandyError> {
    let mut result: Vec<HashMap<String, String>> = Vec::new();
    let dir_items: Vec<FileEntry> = match list_dir_contents(dir){
        Ok(dir_items) => dir_items,
        Err(e) => {
            return Err::<Vec<HashMap<String, String>>, MandyError>(MandyError::new(&e.to_string()));
        }
    };
    for item in dir_items {
        if &item.file_type == &Entity::File
            && item.name.contains(ending) {
            let mut map: HashMap<String, String> = HashMap::new();
            let file_contents: &String = &match read_file(&item.name){
                Ok(file_contents) => file_contents,
                Err(e) => {
                    return Err::<Vec<HashMap<String, String>>, MandyError>(MandyError::new(&e.to_string()));
                }
            };
            let path_list: &Vec<String> = 
                &clean_split(
                    &item.name,
                    &String::from("/")
                );
            let data_file: &String = &path_list[path_list.len()-1];
            let data_file_name_components: &Vec<String> = &clean_split(
                &data_file,
                &String::from(ending)
            );
            let template_key: &String = &data_file_name_components[0];
            map.insert(template_key.to_owned(), file_contents.to_owned());
            result.push(map);
        }
        else {}
    }
    return Ok(result);
}
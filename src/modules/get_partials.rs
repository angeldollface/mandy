/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "dir_is"
/// method from the "coutils"
/// crate to check if a
/// directory exists.
use coutils::dir_is;

/// Importing the "FileEntry"
/// struct to work with files easier.
use coutils::FileEntry;

/// Importing the method
/// from the "coutils"
/// crate to read files
/// into strings.
use coutils::read_file;

/// Getting the function to
/// retrieve variables about
/// Mandy herself.
use super::vars::mandy_vars;

/// Importing Mandy's error
/// struct.
use super::errors::MandyError;

/// Importing Rust's standard
/// "HashMap" API.
use std::collections::HashMap;

/// Importing the method store
/// information about a directory's
/// contents.
use coutils::list_dir_contents;

/// Importing the method to get
/// the base name of a file in a
/// file path.
use super::utils::get_name_base;

/// Attempts to retrieve a list of all Liquid partial templates in the "includes"
/// directory. Returns an error if the directory exists but is empty.
pub fn get_partials(dir: &String) -> Result<Option<HashMap<String, String>>, MandyError> {
    let mut result: HashMap<String, String> = HashMap::new();
    let partials_dir: &String = &mandy_vars()["partials_dir"];
    let includes_dir: &String = &format!("{}/{}", dir, partials_dir);
    if dir_is(includes_dir){
        let file_list: Vec<FileEntry> = list_dir_contents(&includes_dir);
        for file in file_list {
            if file.name.contains(&".liquid"){
                let key: String = get_name_base(&file.name, &String::from(".liquid"))[0].clone();
                let value: String = read_file(&file.name);
                result.insert(key, value);
            }
            else {}
        }
    }
    else {
        return Ok(None);
    }
    if result.is_empty(){
        let e: String = format!("{} exists but is empty.", includes_dir);
        return Err::<Option<HashMap<String, String>>, MandyError>(
            MandyError::new(
                &e.to_string()
            )
        );
    }
    else {}
    return Ok(Some(result));
}
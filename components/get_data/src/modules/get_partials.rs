/*
MANDY DATA by Alexander Abraham a.k.a. "Angel Dollface".
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

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// Importing the method to get
/// the base name of a file in a
/// file path.
use utils::get_name_base;

/// Importing Rust's standard
/// "HashMap" API.
use std::collections::HashMap;

/// Importing the method to store
/// information about a directory's
/// contents.
use coutils::list_dir_contents;

/// Attempts to retrieve a list of all Liquid partial templates in the "partials"
/// directory. Returns an error if the directory exists but is empty.
pub fn get_partials(dir: &String) -> Result<Option<HashMap<String, String>>, MandyError> {
    let mut result: HashMap<String, String> = HashMap::new();
    let partials_dir: &String = &String::from("partials");
    let includes_dir: &String = &format!("{}/{}", dir, partials_dir);
    if dir_is(includes_dir){
        let file_list: Vec<FileEntry> = match list_dir_contents(&includes_dir){
            Ok(file_list) => file_list,
            Err(e) => {
                return Err::<Option<HashMap<String, String>>, MandyError>(
                    MandyError::new(
                        &e.to_string()
                    )
                );
            }
        };
        for file in file_list {
            if file.name.contains(&".liquid"){
                let key: String = get_name_base(&file.name, &String::from(".liquid"))[0].clone();
                let value: String = match read_file(&file.name){
                    Ok(value) => value,
                    Err(e) => {
                        return Err::<Option<HashMap<String, String>>, MandyError>(
                            MandyError::new(
                                &e.to_string()
                            )
                        );
                    }
                };
                result.insert(key, value);
            }
            else {}
        }
    }
    else {
        return Ok(None);
    }
    if result.is_empty(){
        let e: String = format!("\"{}\" exists but is empty.", includes_dir);
        return Err::<Option<HashMap<String, String>>, MandyError>(
            MandyError::new(
                &e.to_string()
            )
        );
    }
    else {}
    return Ok(Some(result));
}
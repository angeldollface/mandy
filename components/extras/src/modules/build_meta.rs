/*
MANDY EXTRAS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "dir_is"
/// method from the "coutils"
/// crate to check if a
/// directory exists.
use coutils::dir_is;

/// Importing the method to get
/// the current time.
use utils::get_time;

/// Importing the method from
/// the "coutils" crate to check
/// whether a file exists.
use coutils::file_is;

/// Importing Mandy's error
/// structure.
use merrors::MandyError;

/// We import the method to create
/// empty text files from the "coutils"
/// crate.
use coutils::create_file;

/// We import the method to write
/// to created files.
use coutils::write_to_file;

/// Importing Rust's standard
/// "HashMap" API.
use std::collections::HashMap;

/// Importing the method from the 
/// "serde_json" crate to generate
/// a pretty JSON string from a 
/// Rust data structure.
use serde_json::to_string_pretty;

/// Generates the file that contains info on the build
/// of a Mandy site.
pub fn generate_meta(dir: &String) -> Result<(), MandyError>{
    let version: String = "0.3.1".to_string();
    let dist_folder: &String = &"dist".to_string();
    let dist_dir: &String = &format!("{}/{}", dir, dist_folder);
    let build_info_file: &String = &format!("{}/meta.json", dist_dir);
    let mut build_meta_data: HashMap<String, String> = HashMap::new();
    build_meta_data.insert(String::from("toolchain"), version);
    build_meta_data.insert(String::from("compiled_at"), get_time());
    let json_string: String = match to_string_pretty(&build_meta_data){
        Ok(json_string) => json_string,
        Err(e) => {return Err::<(), MandyError>(MandyError::new(&e.to_string()));}
    };
    if dir_is(dist_dir){
        if file_is(build_info_file){
            let e: String = format!("{} already exists.", build_info_file);
            return Err::<(), MandyError>(MandyError::new(&e.to_string()));
        }
        else {
            match create_file(&build_info_file){
                Ok(_x) => {},
                Err(e) => {return Err::<(), MandyError>(MandyError::new(&e.to_string()));}
            };
            match write_to_file(build_info_file, &json_string){
                Ok(_x) => {},
                Err(e) => {return Err::<(), MandyError>(MandyError::new(&e.to_string()));}
            };
        }
    }
    else {
        let e: String = format!("{} does not exist.", dist_dir);
        return Err::<(), MandyError>(MandyError::new(&e.to_string()));
    }
    return Ok(())
}
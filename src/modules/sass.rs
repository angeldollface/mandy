/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// We import the "Options"
/// structure from the "grass"
/// crate.
use grass::Options;

/// Importing the "dir_is"
/// method from the "coutils"
/// crate to check if a
/// directory exists.
use coutils::dir_is;

/// We import the "Entity"
/// enum to describe types of files
/// from the "coutils" crate.
use coutils::Entity;

/// Importing the enum to set
/// the output style of the compiled
/// SASS.
use grass::OutputStyle;

/// Importing the method
/// to compile SASS files
/// from the "grass" crate.
use grass::from_path;

/// We import the "FileEntry"
/// structure to store information
/// about files from the "coutils"
/// crate.
use coutils::FileEntry;

/// We import the method to create
/// empty text files from the "coutils"
/// crate.
use coutils::create_file;

/// We import the method to write
/// to created files.
use coutils::write_to_file;

/// Importing Mandy's error
/// struct.
use super::errors::MandyError;

/// Importing the "create_directory"
/// method from the "coutils"
/// crate to create a directory.
use coutils::create_directory;

/// We import the method
/// to list the contents of a 
/// directory.
use coutils::list_dir_contents;

/// Compiles all SASS files in the "sass" directory in a Mandy site.
pub fn compile_sass_files(dir: &String) -> Result<(), MandyError> {
    let sass_dir: &String = &format!("{}/sass", dir);
    if dir_is(sass_dir) {
        let mut sass_files: Vec<FileEntry> = match find_sass_files(sass_dir){
            Ok(sass_files) => sass_files,
            Err(e) => {
                return Err::<(), MandyError>(
                    MandyError::new(&e.to_string())
                );
            }
        };
        if sass_files.is_empty(){
            let err_msg: &String = &format!("\"{}/sass\" empty! Aborting SASS compilation.", dir);
            return Err::<(), MandyError>(
                MandyError::new(&&err_msg.to_string())
            );
        }
        else {
            let mut sass_strings: Vec<String> = Vec::new();
            for sass_file in sass_files {
                let mut compiled = match from_path(&sass_file.name, &Options::default().style(OutputStyle::Compressed)) {
                    Ok(compiled) => compiled,
                    Err(e) => {
                        return Err::<(), MandyError>(
                            MandyError::new(&e.to_string())
                        );
                    }
                };
                sass_strings.push(compiled);
            }
            let css_string: &String = &sass_strings.join("");
            let dist_path: &String = &format!("{}/dist/css", dir);
            let css_file: &String = &format!("{}/index.css", dist_path);
            if dir_is(dist_path){
                let err_msg: String = String::from("\"dist\" directory exists! Aborting SASS compilation.");
                return Err::<(), MandyError>(
                    MandyError::new(&err_msg.to_string())
                );
            }
            else {
                match create_directory(dist_path){
                    Ok(_x) => {},
                    Err(e) => {
                        return Err::<(), MandyError>(
                            MandyError::new(&e.to_string())
                        );
                    }
                };
                match create_file(css_file){
                    Ok(_x) => {},
                    Err(e) => {
                        return Err::<(), MandyError>(
                            MandyError::new(&e.to_string())
                        );
                    }
                };
                match write_to_file(css_file, &css_string){
                    Ok(_x) => {},
                    Err(e) => {
                        return Err::<(), MandyError>(
                            MandyError::new(&e.to_string())
                        );
                    }
                };
            }
        }      
    }
    else {
        let err_msg: &String = &format!("\"{}/sass\" does not exist! Aborting SASS compilation.", dir);
        return Err::<(), MandyError>(
            MandyError::new(&&err_msg.to_string())
        );
    }
    return Ok(());
}

/// Attempts to return a list of all detected files in the "SASS"
/// directory of a Mandy site. 
pub fn find_sass_files(path: &String) -> Result<Vec<FileEntry>, MandyError> {
    let file_ending: String = String::from("");
    let mut result: Vec<FileEntry> = Vec::new();
    if dir_is(path){
        let mut dir_contents: Vec<FileEntry> = match list_dir_contents(path){
            Ok(dir_contents) => dir_contents,
            Err(e) => {
                return Err::<Vec<FileEntry>, MandyError>(
                    MandyError::new(&e.to_string())
                );
            }
        };
        for content in dir_contents {
            if content.file_type == Entity::Dir {
                let mut sub_dir_contents: Vec<FileEntry> = match list_dir_contents(&content.name){
                    Ok(sub_dir_contents) => sub_dir_contents,
                    Err(e) => {
                        return Err::<Vec<FileEntry>, MandyError>(
                            MandyError::new(&e.to_string())
                        );
                    }
                };
                for sub_dir_content in sub_dir_contents {
                    if sub_dir_content.file_type == Entity::Dir {
                        // Do nothing.
                    }
                    else {
                        if sub_dir_content.name.contains(".scss"){
                            result.push(sub_dir_content);
                        }
                        else {}
                    }
                }

            }
            else {
                if content.name.contains(".scss"){
                    result.push(content);
                }
                else {}
            }
        }
    }
    else {}
    return Ok(result);
}
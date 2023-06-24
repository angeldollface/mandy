/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// We import the "options"
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
        let sass_files: Vec<FileEntry> = find_sass_files(sass_dir);
        let mut sass_strings: Vec<String> = Vec::new();
        for sass_file in sass_files {
            let mut compiled = match from_path(&sass_file.name, &Options::default()) {
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
            let err_msg: &String = &String::from("\"dist\" directory exists! Aborting SASS compilation.");
            return Err::<(), MandyError>(
                MandyError::new(&&err_msg.to_string())
            );
        }
        else {
            create_directory(dist_path);
            create_file(css_file);
            write_to_file(css_file, &css_string);
        }      
    }
    else {}
    return Ok(());
}

/// Attempts to return a list of all detected files in the "SASS"
/// directory of a Mandy site. 
pub fn find_sass_files(path: &String) -> Vec<FileEntry> {
    let mut result: Vec<FileEntry> = Vec::new();
    if dir_is(path){
        let dir_contents: Vec<FileEntry> = list_dir_contents(path);
        for content in dir_contents {
            if content.file_type == Entity::Dir {
                let sub_dir_contents: Vec<FileEntry> = list_dir_contents(&content.name);
                for sub_dir_content in sub_dir_contents {
                    if sub_dir_content.file_type == Entity::Dir {
                        // Do nothing.
                    }
                    else {
                        result.push(sub_dir_content);
                    }
                }

            }
            else {
                result.push(content);
            }
        }
    }
    else {}
    return result;
}
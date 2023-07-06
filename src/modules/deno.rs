/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "dir_is"
/// method from the "coutils"
/// crate to check if a
/// directory exists.
use coutils::dir_is;

/// Importing the "file_is"
/// method from the "coutils"
/// crate to check if a
/// file exists.
use coutils::file_is;

/// We import the method to create
/// empty text files from the "coutils"
/// crate.
use coutils::create_file;

/// We import the method to write
/// to created files.
use coutils::write_to_file;

/// Getting the function to
/// retrieve variables about
/// Mandy herself.
use super::vars::mandy_vars;

/// Importing Mandy's error
/// struct.
use super::errors::MandyError;

/// Generates a Typescript file to serve a static 
/// site in $project/dist.
pub fn generate_server(dir: &String) -> Result<(), MandyError> {
    let dist_path: &String = &format!("{}/dist",dir);
    if dir_is(dist_path){
        let server_mod_url: &String = &mandy_vars()["server_mod_url"];
        let code: &String = &format!("import {{ serveSite }} from \'{}\';\nserveSite(\'index.html\').then(() => {{}});", server_mod_url);
        let server_path: &String = &format!("{}/{}", dist_path, mandy_vars()["server_file"]);
        if file_is(server_path){
            let e: String = format!("\"{}\" already exists.", &server_path);
            return Err::<(), MandyError>(
                MandyError::new(
                    &e.to_string()
                )
            );
        }
        else {
            create_file(server_path);
            write_to_file(server_path, code);
        }
    }
    else {
        let e: String = format!("\"{}\" does not exist.", &dist_path);
        return Err::<(), MandyError>(
            MandyError::new(
                &e.to_string()
            )
        );
    }
    return Ok(());
}
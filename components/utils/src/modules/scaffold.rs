/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "dir_is"
/// method from the "coutils"
/// crate to check if a
/// directory exists.
use coutils::dir_is;

/// Importing the "Repository" structure
/// from the "git2" crate to work with Git
/// repositories.
use git2::Repository;

/// Importing the "create_directory"
/// method from the "coutils"
/// crate to create a directory.
use coutils::create_directory;

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// Scaffolds a new Mandy site by attempting to clone a
/// GitHub repository with a Mandy project inside it.
pub fn scaffold_site(dir: &String, short_url_format: &String) -> Result<(), MandyError> {
    if dir_is(dir){
        let err_msg: &String = &format!("The directory \"{}\" already exists!", dir);
        return Err::<(), MandyError>(MandyError::new(&err_msg.to_string()));
    }
    else {
        match create_directory(dir){
            Ok(_x) => {},
            Err(e) => {return Err::<(), MandyError>(MandyError::new(&e.to_string()));}
        }
        let repo_url: &String = &format!("https://github.com/{}", short_url_format);
        let _repo = match Repository::clone(repo_url, dir) {
            Ok(_repo) => _repo,
            Err(e) => {return Err::<(), MandyError>(MandyError::new(&e.to_string()));}
        };
        return Ok(());
    }
}
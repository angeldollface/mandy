/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "dir_is"
/// method from the "coutils"
/// crate to check if a
/// directory exists.
use coutils::dir_is;

/// Importing the "clean_split"
/// method from the "coutils"
/// method to split strings.
use coutils::clean_split;

/// Importing Rust's standard
/// "HashMap" data structure.
use std::collections::HashMap;

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// Importing the method to get
/// the base name of a file in a
/// file path.
use utils::get_name_base;

/// Importing the data structure to
/// represent all information about
/// a Markdown document in a Mandy site.
use processors::MandyMDDocument;

/// Importing the method to retrieve the data
/// contexts of Markdown documents in a Mandy site.
use super::get_pages::get_page_contexts;

/// Retrieves the contexts for directories the user would like to loop over.
pub fn get_loop_content(dirs: &String, project_dir: &String) -> Result<HashMap<String, Vec<HashMap<String, String>>>, MandyError> {
    let mut result: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    let dirs: Vec<String> = clean_split(dirs, &String::from("|"));
    if dirs.is_empty(){
        let err_msg: String = String::from("No directories for loop content supplied!");
        return Err::<HashMap<String, Vec<HashMap<String, String>>>, MandyError>(
            MandyError::new(
                &err_msg.to_string()
            )
        );
    }
    else {
        for dir in dirs {
            let dir_path: &String = &format!("{}/{}", project_dir, dir);
            if dir_is(&dir_path) || dir_path == &String::from("") {
                let mut res_vec: Vec<HashMap<String, String>> = Vec::new();
                let mut md_files: Vec<MandyMDDocument> = match get_page_contexts(&dir_path) {
                    Ok(md_files) => md_files,
                    Err(e) => {
                        return Err::<HashMap<String, Vec<HashMap<String, String>>>, MandyError>(
                            MandyError::new(
                                &e.to_string()
                            )
                        );
                    }
                };
                if md_files.is_empty(){
                    let err_msg: String = format!("\"{}\" is empty.", dir_path);
                    return Err::<HashMap<String, Vec<HashMap<String, String>>>, MandyError>(
                        MandyError::new(
                            &err_msg.to_string()
                        )
                    );
                }
                else {
                    for md_doc in md_files {
                        let md_file_url: &String = &format!("{}/{}", dir, get_name_base(&md_doc.file, &String::from(".markdown"))[0]);
                        let mut ctx_clone: HashMap<String, String> = md_doc.ctx.clone();
                        ctx_clone.insert(String::from("url"), md_file_url.to_owned());
                        res_vec.push(ctx_clone);
                    }
                    result.insert(dir, res_vec);
                }
            }
            else {
                let err_msg: String = format!("Directory \"{}\" does not exist.", dir_path);
                return Err::<HashMap<String, Vec<HashMap<String, String>>>, MandyError>(
                    MandyError::new(
                        &err_msg.to_string()
                    )
                );
            }
        }
    }
    return Ok(result);
}
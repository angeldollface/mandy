/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the method from
/// the "coutils" crate to read a text
/// file into a string.
use coutils::read_file;

/// Importing a data structure
/// to represent metadata of a Markdown
/// file in a Mandy site.
use super::finder::MDFile;

/// Importing Mandy's error
/// struct.
use super::errors::MandyError;

/// Importing the method to find
/// and detect Markdown files in
/// a Mandy site.
use super::finder::find_md_files;

/// Importing the method to 
/// deserialize Markdown files into
/// data structures Mandy's compiler
/// can use.
use super::markdown::deserialize_md;

/// Importing the data structure to
/// represent all information about
/// a Markdown document in a Mandy site.
use super::markdown::MandyMDDocument;

/// Attempts to retrieve all the information about all
/// Markdown documents in a Mandy site. Only goes two levels
/// deep into a Mandy project. Anything beyond that will be
/// ignored.
pub fn get_page_contexts(
    dir: &String
) ->  Result<Vec<MandyMDDocument>, MandyError> {
    let mut md_documents: Vec<MandyMDDocument> = Vec::new();
    let mut md_files: Vec<MDFile> = match find_md_files(&dir){
        Ok(md_files) => md_files,
        Err(e) => {
            return Err::<Vec<MandyMDDocument>, MandyError>(
                MandyError::new(
                    &e.to_string()
                )
            );
        }
    };
    if md_files.is_empty(){
        let err_msg: String = format!("No Markdown files found in \"{}\"!", dir);
        return Err::<Vec<MandyMDDocument>, MandyError>(
            MandyError::new(
                &err_msg.to_string()
            )
        );
    }
    else {
        for md_file in md_files {
            let md_file_path: String = md_file.file;
            let mut md_string = match read_file(&md_file_path){
                Ok(md_string) => md_string,
                Err(e) => {
                    return Err::<Vec<MandyMDDocument>, MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );
                }
            };
            let mut md_document = match deserialize_md(&md_string){
                Ok(md_document) => md_document,
                Err(e) => {
                    let err: String = format!(
                        "Error found in file \"{}\"\n{}", md_file_path, e
                    );
                    return Err::<Vec<MandyMDDocument>, MandyError>(
                        MandyError::new(
                            &err.to_string()
                        )
                    );
                }
            };
            md_documents.push(
                MandyMDDocument::new(
                    &md_file_path,
                    &md_file.dir,
                    &md_document.to_map()
                )
            );
        }
    }
    return Ok(md_documents);
}
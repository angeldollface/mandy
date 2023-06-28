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

/// Rust's file metadata
/// API from the "fs"
/// module.
use std::fs::metadata;

/// Importing the method from
/// the "coutils" crate to copy
/// a file from source to
/// destination.
use coutils::file_copy;

/// Importing the method from
/// the "coutils" crate to read a text
/// file into a string.
use coutils::read_file;

/// Importing the method from
/// the "coutils" crate to copy
/// a folder from source to
/// destination.
use coutils::folder_copy;

/// We import the method to create
/// empty text files from the "coutils"
/// crate.
use coutils::create_file;

/// We import the method to write
/// to created files.
use coutils::write_to_file;

/// Importing the "create_directory"
/// method from the "coutils"
/// crate to create a directory.
use coutils::create_directory;

/// Importing Mandy's error
/// struct.
use super::errors::MandyError;

/// Importing a data structure
/// to describe and handle files
/// to copy to the compiled Mandy
/// project.
use super::arrow_set::ArrowSet;

/// Importing the method to get
/// the last dir in a file path.
use super::utils::get_last_dir;

/// Importing the method to get
/// the base name of a file in a
/// file path.
use super::utils::get_name_base;

/// Importing the structure representing
/// a Mandy site's context.
use super::context::SiteContext;

/// Importing the method to apply a data
/// context to a Liquid template and
/// render this into an HTML string.
use super::liquid::render_template;

/// Importing the function to parse
/// an arrow set string into the "ArrowSet"
/// data structure.
use super::arrow_set::parse_arrow_set;


/// Creates a file and renders it from a single
/// "SiteContext" instance.
pub fn build_context(ctx: &SiteContext, dir: &String) -> Result<(), MandyError> {
    let dist_path: &String = &format!("{}/dist", dir);
    if dir_is(dist_path){
        // Don't create $dir/dist again.
    }
    else {
        create_directory(dist_path);
    }
    if ctx.page.contains_key("layout") {
        let layout_path: &String = &format!("{}/{}", dir, ctx.page["layout"]);
        if file_is(&layout_path) {
            let liquid_string: &String = &read_file(&layout_path);
            let last_md_dir: &String = &get_last_dir(&ctx.file);
            let name_base: &String = &get_name_base(&ctx.file)[0];
            let html_name: &String = &format!("index.html");
            let mut html_path: String = String::from("");
            if last_md_dir == dir {
                html_path = format!("{}/{}", dist_path, html_name);
            }
            else {
                let md_path: &String = &format!("{}/dist/{}", dir, last_md_dir);
                let page_path: &String = &format!("{}/{}", &md_path, &name_base);
                html_path = format!("{}/{}", page_path, html_name);
                if dir_is(md_path){
                    if dir_is(page_path){}
                    else {create_directory(page_path);}
                }
                else {
                    create_directory(md_path);
                    if dir_is(page_path){}
                    else {create_directory(page_path);}
                }
            }
            let mut html_string = match render_template(&liquid_string, ctx) {
                Ok(html_string) => html_string,
                Err(e) => {
                    return Err::<(), MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );
                }
            };
            let mut arrow_set: ArrowSet = match parse_arrow_set(&ctx.copy_files){
                Ok(arrow_set) => arrow_set,
                Err(e) => {
                    return Err::<(), MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );
                }
            };
            if arrow_set.flag == String::from("true"){
                let files_to_copy: Vec<String> = arrow_set.set.clone();
                for file in files_to_copy {
                    if file == String::from(""){
                        let msg: &String = &format!("Invalid character sequence found in the \"copyFiles\" flag!");
                        return Err::<(), MandyError>(MandyError::new(&msg.to_string()));
                    }
                    else {
                        let file_path: &String = &format!("{}/{}", dir, file);
                        let copied_assets_path: &String = &format!("{}/{}", dist_path, file);
                        let meta = match metadata(file_path){
                            Ok(meta) => meta,
                            Err(e) => {
                                let msg: &String = &format!("File \"{}\" caused the error:\n{}", file_path, e);
                                return Err::<(), MandyError>(
                                    MandyError::new(
                                        &msg.to_string()
                                    )
                                );
                            }
                        };
                        if dir_is(file_path) || file_is(file_path) {
                            if dir_is(copied_assets_path) || file_is(copied_assets_path) {}
                            else {
                                if meta.is_dir() {
                                    folder_copy(file_path, dist_path);                        
                                }
                                else if meta.is_file() {
                                    file_copy(file_path, copied_assets_path);                                                                        
                                }
                            }
                        }
                        else {
                            let err_msg: String = format!("Path \"{}\" not found.", file_path);
                            return Err::<(), MandyError>(MandyError::new(&err_msg.to_string()));
                        }
                    }
                }
            }
            else {
                // Do nothing.
            }
            create_file(&html_path);
            write_to_file(&html_path, &html_string);
        }
        else {
            let err_msg: String = format!("Layout for \"{}\" supllied but not found.", ctx.file);
            return Err::<(), MandyError>(
                MandyError::new(
                    &err_msg.to_string()
                )
            );
        }
    }
    else {
        let err_msg: String = format!("No layout supplied for \"{}\"!", ctx.file);
        return Err::<(), MandyError>(
            MandyError::new(
                &err_msg.to_string()
            )
        );
    }
    return Ok(());
}

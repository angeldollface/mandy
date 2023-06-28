/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "dir_is"
/// method from the "coutils"
/// crate to check if a
/// directory exists.
use coutils::dir_is;

/// Importing Mandy's error
/// struct.
use super::errors::MandyError;

/// Importing the structure representing
/// a Mandy site's context.
use super::context::SiteContext;

/// Importing the method to compile
/// SASS files into a CSS file from
/// "./sass.rs".
use super::sass::compile_sass_files;

/// Importing the function to build a single
/// site context from "./build_context.rs".
use super::build_context::build_context;

/// A function to retrieve site contexts from
/// a Mandy site project.
use super::get_context::get_site_contexts;

/// Creates files and renders them from all
/// "SiteContext" instances.
pub fn compile_site(dir: &String) -> Result<(), MandyError> {
    if dir_is(dir) {
        let mut site_contexts: Vec<SiteContext> = match get_site_contexts(dir) {
            Ok(site_contexts) => site_contexts,
            Err(e) => {
                return Err::<(), MandyError>(
                    MandyError::new(
                        &e.to_string()
                    )
                );
            }
        };
        for ctx in site_contexts {
            let build_op: Result<(), MandyError> = build_context(&ctx, dir);
            match build_op {
                Ok(_x) => {},
                Err(e) => {
                    return Err::<(), MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );
                }
            };
        }
        match compile_sass_files(dir) {
            Ok(_x) => {},
            Err(e) => {
                return Err::<(), MandyError>(
                    MandyError::new(
                        &e.to_string()
                    )
                );
            }
        }
    }
    else {
        let err_msg: String = format!("\"{}\" not found.", dir);
        return Err::<(), MandyError>(
            MandyError::new(
                &err_msg.to_string()
            )
        );
    }
    return Ok(());
}
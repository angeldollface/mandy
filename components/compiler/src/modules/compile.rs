/*
MANDY COMPILER by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "dir_is"
/// method from the "coutils"
/// crate to check if a
/// directory exists.
use coutils::dir_is;

/// Importing the method from
/// the "utils" module to clean
/// any file URLs. We need this
/// for sitemap generation.
use utils::clean_url;

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// Importing the structure representing
/// a Mandy site's context.
use context::SiteContext;

/// Importing the method to generate the
/// build meta data file.
use extras::generate_meta;

/// Importing the function to generate
/// a server file for Deno Deploy.
use extras::generate_server;

/// Importing the method to compile
/// SASS files into a CSS file.
use extras::compile_sass_files;

/// A function to retrieve site contexts from
/// a Mandy site project.
use get_data::get_site_contexts;

/// Importing the method to create files for
/// SEO and search engine crawlers.
use extras::create_crawler_files;

/// Importing the function to build a single
/// site context from "./build_context.rs".
use super::build_context::build_context;

/// Creates files and renders them from all
/// "SiteContext" instances.
pub fn compile_site(dir: &String) -> Result<(), MandyError> {
    let dist_path: &String = &format!("{}/dist", dir);
    if dir_is(dist_path){
        let msg: String = format!("\"{}\" already exists!", dist_path);
        return Err::<(), MandyError>(
            MandyError::new(
                &msg.to_string()
            )
        );
    }
    else {
        if dir_is(dir) {
            let site_contexts: Vec<SiteContext> = match get_site_contexts(dir) {
                Ok(site_contexts) => site_contexts,
                Err(e) => {
                    return Err::<(), MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );
                }
            };
            let mut urls: Vec<String> = Vec::new();
            let mut tl_domain: String = String::from("");
            let mut baseurl: String = String::from("");
            let mut freq: String = String::from("");
            for ctx in site_contexts {
                urls.push(clean_url(&ctx.clone().file, dir, &ctx.clone().dir));
                tl_domain = ctx.clone().site["tlDomain"].clone();
                baseurl = ctx.clone().site["baseurl"].clone();
                freq = ctx.clone().site["updateFreq"].clone();
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
            };
            match create_crawler_files(&urls, &freq, &baseurl, &tl_domain, dir) {
                Ok(_x) => {},
                Err(e) => {
                    return Err::<(), MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );
                }
            }
            match generate_meta(dir){
                Ok(_x) => {},
                Err(e) => {
                    return Err::<(), MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );
                }
            };
            match generate_server(dir){
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
    }
    return Ok(());
}

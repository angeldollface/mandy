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

/// Importing the method to get
/// the base name of a file in a
/// file path.
use super::utils::get_name_base;

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

/// A method to create the files for
/// crawling bots of search engines.
/// Provides hypersonic SEO for Mandy-powered
/// sites.
use super::crawlers::create_crawler_files;

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
            let mut urls: Vec<String> = Vec::new();
            let mut tl_domain: String = String::from("");
            let mut baseurl: String = String::from("");
            let mut freq: String = String::from("");
            for ctx in site_contexts {
                tl_domain = ctx.clone().site["tlDomain"].clone();
                freq = ctx.clone().site["updateFreq"].clone();
                baseurl = ctx.clone().baseurl;
                let file_url: &String = &ctx.file;
                let mut dir_url: String = ctx.clone().dir;
                let fname: String = get_name_base(file_url)[0].clone();
                dir_url = dir_url.replace(dir, &"");
                let mut final_url: String = format!("{}/{}", &dir_url, fname);
                if &dir_url == &String::from(""){
                    final_url = String::from("/");
                }
                else {}
                urls.push(final_url.clone());
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
            match create_crawler_files(
                &urls, 
                &freq,
                &baseurl, 
                &tl_domain, 
                dir
            ){
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
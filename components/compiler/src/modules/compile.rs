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

/// Gets the current time
/// as a string.
use coutils::get_time;

/// Imports the data structure
/// to hold log message information.
use extras::LogMessage;

/// Imports the function
/// to create a build log.
use extras::create_log;

/// Imports the function
/// to create a JSON API
/// of iterative content.
use extras::create_api;

/// Importing the function to 
/// make an RSS feed
/// XML file.
use extras::create_feed;

/// Importing Mandy's error
/// structure.
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
/// site context.
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
            let mut logging_items: Vec<LogMessage> = Vec::new();
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
            let mut build_log_on: bool = false;
            let mut iter_content_on: bool = false;
            for ctx in &site_contexts {
                let path: &String = &clean_url(&ctx.clone().file, dir, &ctx.clone().dir);
                urls.push(path.to_owned());
                tl_domain = ctx.clone().site["tlDomain"].clone();
                baseurl = ctx.clone().site["baseurl"].clone();
                freq = ctx.clone().site["updateFreq"].clone();
                if ctx.clone().site["build_log"].clone() == String::from("true"){
                    build_log_on = true;
                }
                else {}
                if ctx.clone().site["hasLoopContent"].clone() == String::from("true") {
                    iter_content_on = true;
                }
                else {}
                let build_op: Result<(), MandyError> = build_context(&ctx, dir);
                match build_op {
                    Ok(_x) => {
                        logging_items.push(
                            LogMessage::new(
                                &format!("Building file \"{}\"...", &path),
                                &get_time(),
                                &dir
                            )
                        );
                    },
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
                Ok(_x) => {
                    logging_items.push(
                        LogMessage::new(
                            &format!("Building SASS!"),
                            &get_time(),
                            &dir
                        )
                    );
                },
                Err(e) => {
                    return Err::<(), MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );
                }
            };
            match create_crawler_files(&urls, &freq, &baseurl, &tl_domain, dir) {
                Ok(_x) => {
                    logging_items.push(
                        LogMessage::new(
                            &"Creating crawler files.",
                            &get_time(),
                            &dir
                        )
                    );
                },
                Err(e) => {
                    return Err::<(), MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );
                }
            };
            match generate_meta(dir){
                Ok(_x) => {
                    logging_items.push(
                        LogMessage::new(
                            &"Generating metadata.",
                            &get_time(),
                            &dir
                        )
                    );
                },
                Err(e) => {
                    return Err::<(), MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );
                }
            };
            match generate_server(dir){
                Ok(_x) => {
                    logging_items.push(
                        LogMessage::new(
                            &"Generating Typescript server file.",
                            &get_time(),
                            &dir
                        )
                    );
                },
                Err(e) => {
                    return Err::<(), MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );
                }
            };
            if iter_content_on {
                match create_feed(dir, &site_contexts){
                    Ok(_x) => {
                        logging_items.push(
                            LogMessage::new(
                                &"Generating RSS feed.",
                                &get_time(),
                                &dir
                            )
                        );
                    },
                    Err(e) => {
                        return Err::<(), MandyError>(
                            MandyError::new(
                                &e.to_string()
                            )
                        );
                    }
                };
                match create_api(dir, &site_contexts){
                    Ok(_x) => {
                        logging_items.push(
                            LogMessage::new(
                                &"Generating JSON API.",
                                &get_time(),
                                &dir
                            )
                        );
                    },
                    Err(e) => {
                        return Err::<(), MandyError>(
                            MandyError::new(
                                &e.to_string()
                            )
                        );
                    }
                };
            }
            else {}
            if build_log_on {
                match create_log(&logging_items, dir){
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
            else {}
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
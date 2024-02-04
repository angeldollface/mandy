/*
MANDY DATA by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the method from
/// the "coutils" crate to check
/// whether a file exists.
use coutils::file_is;

use extras::build_categories;
/// Importing the method to check
/// the environment in which a Mandy
/// site is being compiled.
use utils::detect_env;

/// Importing the method from
/// the "coutils" crate to read a text
/// file into a string.
use coutils::read_file;

/// Importing the "Environment"
/// enum to quantify the different
/// types of Mandy compilation
/// environments.
use utils::Environment;

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// Importing the data structure to
/// represent a data context of a 
/// Mandy site.
use context::SiteContext;

/// Importing the method
/// to retrieve data objects
/// from a Mandy site if they
/// exist.
use super::get_data::get_data;

/// Importing Rust's standard
/// "HashMap" API.
use std::collections::HashMap;

/// Importing the data structure to
/// represent all information about
/// a Markdown document in a Mandy site.
use processors::MandyMDDocument;

/// Importing the method to deserialize and
/// read the configuration file in JSON format
/// of a Mandy site.
use processors::deserialize_config_json;

/// Importing the method to deserialize and
/// read the configuration file in TOML format
/// of a Mandy site.
use processors::deserialize_config_toml;

/// Importing the method to deserialize and
/// read the configuration file in YAML format
/// of a Mandy site.
use processors::deserialize_config_yaml;

/// Importing the method to get information
/// on Liquid partial templates to parse and render
/// Liquid templates in a Mandy site.
use super::get_partials::get_partials;

/// Importing the method to retrieve the data
/// contexts of Markdown documents in a Mandy site.
use super::get_pages::get_page_contexts;

/// Importing the method to retrieve the contexts
/// for content the user would like to loop over.
use super::get_loop_content::get_loop_content;

/// Attempts to retrieve the site context for each ".markdown"
/// file in a Mandy site.
pub fn get_site_contexts(dir: &String) -> Result<Vec<SiteContext>, MandyError> {
    let mut result: Vec<SiteContext> = Vec::new(); 
    if file_is(&format!("{}/config.yaml", &dir)) || 
        file_is(&format!("{}/config.toml", &dir)) ||
        file_is(&format!("{}/config.json", &dir))
    {
        let config_data: HashMap<String,String>;
        if file_is(&format!("{}/config.yaml", &dir)){
            let config_string: String = match read_file(&format!("{}/config.yaml", &dir)){
                Ok(config_string) => config_string,
                Err(e) => {
                    return Err::<Vec<SiteContext>, MandyError>(MandyError::new(&e.to_string()));
                }
            };
            config_data = match deserialize_config_yaml(
                &config_string) {
                Ok(config_data) => config_data,
                Err(e) => {let err_msg: String = format!("Error in config:\n{}", e);return Err::<Vec<SiteContext>, MandyError>(MandyError::new(&err_msg.to_string()));}
            };
        }
        else if file_is(&format!("{}/config.toml", &dir)){
            let config_string: String = match read_file(&format!("{}/config.toml", &dir)){
                Ok(config_string) => config_string,
                Err(e) => {
                    return Err::<Vec<SiteContext>, MandyError>(MandyError::new(&e.to_string()));
                }
            };
            config_data = match deserialize_config_toml(
                &config_string) {
                Ok(config_data) => config_data,
                Err(e) => {let err_msg: String = format!("Error in config:\n{}", e);return Err::<Vec<SiteContext>, MandyError>(MandyError::new(&err_msg.to_string()));}
            };
        }
        else if file_is(&format!("{}/config.json", &dir)){
            let config_string: String = match read_file(&format!("{}/config.json", &dir)){
                Ok(config_string) => config_string,
                Err(e) => {
                    return Err::<Vec<SiteContext>, MandyError>(MandyError::new(&e.to_string()));
                }
            };
            config_data = match deserialize_config_json(
                &config_string) {
                Ok(config_data) => config_data,
                Err(e) => {let err_msg: String = format!("Error in config:\n{}", e);return Err::<Vec<SiteContext>, MandyError>(MandyError::new(&err_msg.to_string()));}
            };
        }
        else {
            let e: String = "A config file could not be found or could not be parsed.".to_string();
            return Err::<Vec<SiteContext>, MandyError>(MandyError::new(&e.to_string()));
        }
        if config_data.contains_key(&String::from("prod_url")) &&
           config_data.contains_key(&String::from("dev_url")) &&
           config_data.contains_key(&String::from("hasLoopContent")) &&
           config_data.contains_key(&String::from("copyFiles")) &&
           config_data.contains_key(&String::from("tlDomain")) &&
           config_data.contains_key(&String::from("updateFreq")) &&
           config_data.contains_key(&String::from("description")) &&
           config_data.contains_key(&String::from("build_log")){

            let baseurl;
            let comp_env: Environment = match detect_env() {
                Ok(comp_env) => comp_env,
                Err(e) =>{return Err::<Vec<SiteContext>, MandyError>(MandyError::new(&e.to_string()));}
            };
            if comp_env == Environment::Development {baseurl = &config_data["dev_url"];}
            else {baseurl = &config_data["prod_url"];}
            let page_contexts: Vec<MandyMDDocument> = match get_page_contexts(dir){
                Ok(page_contexts) => page_contexts,
                Err(e) => {
                    return Err::<Vec<SiteContext>, MandyError>(MandyError::new(&e.to_string()));
                }
            };
            let fetched_data = match get_data(dir){
                Ok(fetched_data) => fetched_data,
                Err(e) => {
                    return Err::<Vec<SiteContext>, MandyError>(MandyError::new(&e.to_string()));
                }
            };
            let partials: Option<HashMap<String, String>> = match get_partials(dir) {
                Ok(partials) => partials,
                Err(e) => {
                    return Err::<Vec<SiteContext>, MandyError>(MandyError::new(&e.to_string()));
                }
            };
            for page_context in page_contexts {
                let mut config_clone: HashMap<String, String> = config_data.clone();
                config_clone.insert(String::from("baseurl"), baseurl.to_owned());
                if config_data["hasLoopContent"] == String::from("true") {


                    if config_data.contains_key("loopContentDirs"){
                        let dirs: &String = &config_data["loopContentDirs"];
                        let loop_contexts = match get_loop_content(
                            dirs,
                            dir
                        ){
                            Ok(loop_contexts) => loop_contexts,
                            Err(e) => {
                                return Err::<Vec<SiteContext>, MandyError>(
                                    MandyError::new(
                                        &e.to_string()
                                    )
                                );
                            }
                        };
                        let post_cats = build_categories(&loop_contexts);
                        if post_cats.is_empty(){
                            let ctx: SiteContext = SiteContext::new(&config_data["copyFiles"], &partials, &baseurl, &config_clone, &page_context.ctx, &page_context.file, &page_context.dir,&Some(loop_contexts), &fetched_data, &None);
                            result.push(ctx);
                        }
                        else {
                            let ctx: SiteContext = SiteContext::new(&config_data["copyFiles"], &partials, &baseurl, &config_clone, &page_context.ctx, &page_context.file, &page_context.dir,&Some(loop_contexts), &fetched_data, &Some(post_cats));
                            result.push(ctx);
                        }
                    }
                    else {
                        let err_msg: &String = &format!("\"hasLoopContent\" set to \"true\" but no directories specified.");
                        return Err::<Vec<SiteContext>, MandyError>(
                            MandyError::new(
                                &err_msg.to_string()
                            )
                        );
                    }
                }
                else {
                    let ctx: SiteContext = SiteContext::new(&config_data["copyFiles"], &partials, &baseurl, &config_clone, &page_context.ctx, &page_context.file, &page_context.dir,&None, &fetched_data, &None);
                    result.push(ctx);
                }
            }
        }
        else {
            let err_msg: &String = &format!("One or all of the following config flags not found:\n- \"prod_url\"\n- \"dev_url\"\n- \"copyFiles\"\n- \"hasLoopContent\"\n- \"tlDomain\"\n- \"updateFreq\"\n- \"description\"\n- \"build_log\"");
            return Err::<Vec<SiteContext>, MandyError>(
                MandyError::new(
                    &err_msg.to_string()
                )
            );
        }
    }
    else {
        let err_msg: &String = &format!("Config file not found.");
        return Err::<Vec<SiteContext>, MandyError>(
            MandyError::new(
                &err_msg.to_string()
            )
        );
    }
    return Ok(result);
}
/*
MANDY EXTRAS by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the function
/// from the "coutils" crate
/// to check whether a file
/// exists.
use coutils::file_is;

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// Importing the structure representing
/// a Mandy site's context.
use context::SiteContext;

/// Importing the function
/// from the "coutils" crate
/// to create a file.
use coutils::create_file;

/// Importing the function
/// from the "coutils" crate
/// to write text to a file.
use coutils::write_to_file;

/// Importing the method from the
/// "serde_json" crate to serialize
/// a Rust data structure into JSON
/// code.
use serde_json::to_string_pretty;

/// Importing the method to eliminate
/// duplicates from a vector of site data.
use super::rss::loop_content_is_same;

/// This method creates a JSON API
/// of iterative content and puts this
/// into a JSON file. Returns an error
/// if this operation fails.
pub fn create_api(
    dir: &String,
    site_contexts: &Vec<SiteContext>
) -> Result<(), MandyError>{
    let api_path: String = format!("{}/dist/api.json", &dir);
    if loop_content_is_same(site_contexts) && !(site_contexts.is_empty()){
        let site_context = &site_contexts[0].clone();
        match &site_context.loop_content{
            Some(content) => {
                let json_str: String = match to_string_pretty(&content){
                    Ok(json_str) => json_str,
                    Err(e) => {
                        return Err::<(), MandyError>(MandyError::new(&e.to_string()));
                    }
                };
                if file_is(&api_path){
                    let e: String = format!("File already exists at: \"{}\"!", &api_path);
                    {return Err::<(), MandyError>(MandyError::new(&e.to_string()));}
                }
                else {
                    match create_file(&api_path){
                        Ok(_y) => {
                            match write_to_file(
                                &api_path,
                                &json_str
                            ){
                                Ok(_c) => {},
                                Err(e) => {
                                    {return Err::<(), MandyError>(MandyError::new(&e.to_string()));}
                                }
                            }
                        },
                        Err(e) => {
                            {return Err::<(), MandyError>(MandyError::new(&e.to_string()));}
                        }
                    }
            }
            },
            None => {}
        };
    }
    else {
        let e: String = format!("Could not gather site data.\n");
        return Err::<(), MandyError>(MandyError::new(&e.to_string()));        
    }
    Ok(())
}
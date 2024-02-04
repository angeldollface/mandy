/*
MANDY EXTRAS by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the function
/// from the "coutils" crate
/// to check whether a
/// directory exists.
use coutils::dir_is;

/// Importing the function
/// from the "coutils" crate
/// to check whether a file
/// exists.
use coutils::file_is;

/// Importing the "Item"
/// data structure from the
/// RSS handler to use here.
use super::rss::Item;

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// Importing the "Channel"
/// data structure from the
/// RSS handler to use here.
use super::rss::Channel;

/// Importing the "RSSFeed"
/// data structure from the
/// RSS handler to use here.
use super::rss::RSSFeed;

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
    let dist_dir_path: String = format!("{}/dist", dir);
    if dir_is(&dist_dir_path){
        let rss_path: String = format!("{}/dist/api.json", &dir);
        let mut channels: Vec<Channel> = Vec::new(); 
        if loop_content_is_same(site_contexts) && !(site_contexts.is_empty()){
            let site_context = &site_contexts[0].clone();
            match &site_context.loop_content{
                Some(content) => {
                    for (channel, items) in content {
                        let mut rss_items: Vec<Item> = Vec::new();
                        for item in items {
                            if !(&site_context.site.contains_key("tlDomain")){
                                let e: String = String::from("The \"tlDomain\" flag was not specified!");
                                return Err::<(), MandyError>(MandyError::new(&e.to_string()));
                            }
                            let tl_domain: &String = &site_context.site["tlDomain"];
                            let rss_item: Item = Item::new(
                                &item["title"],
                                &item["description"],
                                &item["date"],
                                &format!("{}{}{}", tl_domain, &site_context.baseurl, item["url"])
                            );
                            rss_items.push(rss_item);
                        }
                        channels.push(
                            Channel::new(
                                &channel,
                                &site_context.site["description"],
                                &site_context.baseurl,
                                &rss_items
                            )
                        );
                    }
                },
                None => {}
            };
        }
        else {
            let e: String = format!("Could not gather site data.\n");
            return Err::<(), MandyError>(MandyError::new(&e.to_string()));        
        }
        if channels.is_empty(){}
        else {
            if file_is(&rss_path){
                let e: String = format!("File already exists at: \"{}\"!", &rss_path);
                return Err::<(), MandyError>(MandyError::new(&e.to_string()));
            }
            else {
                let json_str: String = match to_string_pretty(&RSSFeed::new(&channels)){
                    Ok(json_str) => json_str,
                    Err(e) => {
                        return Err::<(), MandyError>(MandyError::new(&e.to_string()));
                    }
                };
                match create_file(&rss_path){
                Ok(_y) => {
                    match write_to_file(
                        &rss_path,
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
    }}
    else {
        let e: String = format!("The path \"{}\" does not exist!", &dist_dir_path);
        return Err::<(), MandyError>(MandyError::new(&e.to_string()));
    }
    Ok(())
}
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

/// We import the method to create
/// empty text files from the "coutils"
/// crate.
use coutils::create_file;

/// We import the method to write
/// to created files.
use coutils::write_to_file;

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// Importing the method to create the
/// string for "sitemap.xml".
use super::site_map::create_site_map;

/// Importing the method to create the
/// string for "robots.txt".
use super::robots::create_robots_txt;

/// Creates the files "$project/dist/sitemap.xml"
/// and "$project/dist/robots.txt".
pub fn create_crawler_files(
    urls: &Vec<String>,
    freq: &String,
    baseurl: &String, 
    top_level_domain: &String,
    dir: &String
) -> Result<(), MandyError> {
    let dist_dir: &String = &format!("{}/dist", dir);
    let robots_txt_file_path = &format!("{}/robots.txt", dist_dir);
    let site_map_xml_file_path = &format!("{}/sitemap.xml", dist_dir);
    let robots_txt_contents: &String = &create_robots_txt(top_level_domain, baseurl);
    let site_map_xml_contents: &String = &create_site_map(urls, freq, baseurl, top_level_domain);
    if dir_is(dist_dir){
        if file_is(robots_txt_contents){}
        else {
            match create_file(robots_txt_file_path){
                Ok(_x) => {},
                Err(e) => {return Err::<(), MandyError>(MandyError::new(&e.to_string()));}
            };
            match write_to_file(robots_txt_file_path, robots_txt_contents){
                Ok(_x) => {},
                Err(e) => {return Err::<(), MandyError>(MandyError::new(&e.to_string()));}
            };
        }
        if file_is(site_map_xml_file_path){}
        else {
            match create_file(site_map_xml_file_path){
                Ok(_x) => {},
                Err(e) => {return Err::<(), MandyError>(MandyError::new(&e.to_string()));}
            }
            match write_to_file(site_map_xml_file_path, site_map_xml_contents){
                Ok(_x) => {},
                Err(e) => {return Err::<(), MandyError>(MandyError::new(&e.to_string()));}
            }
        }
    }
    else {
        let e: String = format!(
            "\"{}\" already exists!\nAborting creation of \"sitemap.xml\" and \"robots.txt\"",
            dist_dir
        );
        return Err::<(), MandyError>(
            MandyError::new(
                &e.to_string()
            )
        );
    }
    return Ok(());
}
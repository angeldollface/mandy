/*
MANDY SITE CONTEXT by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing Rust's
/// standard "Debug"
/// trait.
use std::fmt::Debug;

/// Importing Liquid's
/// standard "ValueView"
/// trait.
use liquid::ValueView;

/// Importing Liquid's
/// standard "ObjectView"
/// trait.
use liquid::ObjectView;

/// Importing Rust's standard
/// "HashMap" API.
use std::collections::HashMap;

/// A data structure
/// to hold different
/// types of data for
/// a Mandy site.
#[derive(ObjectView, ValueView, Debug, Clone)]
pub struct SiteContext {
    pub copy_files: String,
    pub partial_templates: Option<HashMap<String, String>>,
    pub baseurl: String,
    pub site: HashMap<String, String>,
    pub page: HashMap<String, String>,
    pub file: String,
    pub dir: String,
    pub loop_content: Option<HashMap<String, Vec<HashMap<String, String>>>>,
    pub data: Option<HashMap<String, Vec<HashMap<String, String>>>>,
}

/// Implementing generic methods
/// for the "SiteContext"
/// data structure.
impl SiteContext {

    /// A method to create a new instance
    /// of the "SiteContext" data
    /// structure.
    pub fn new(
        copy_files: &String,
        partial_templates: &Option<HashMap<String, String>>,
        baseurl: &String,
        site: &HashMap<String, String>,
        page: &HashMap<String, String>,
        file: &String,
        dir: &String,
        loop_content: &Option<HashMap<String, Vec<HashMap<String, String>>>>,
        data: &Option<HashMap<String, Vec<HashMap<String, String>>>>
    ) -> SiteContext {
        return SiteContext { 
            copy_files: copy_files.to_owned(),
            partial_templates: partial_templates.to_owned(),
            baseurl: baseurl.to_owned(), 
            site: site.to_owned(), 
            page: page.to_owned(), 
            file: file.to_owned(), 
            dir: dir.to_owned(), 
            loop_content: loop_content.to_owned(),
            data: data.to_owned()
        };
    }
}
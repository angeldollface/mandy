/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the 
/// "to_html" method
/// from the "markdown"
/// crate.
use markdown::to_html;

/// Importing Rust's standard
/// "HashMap" data structure.
use std::collections::HashMap;

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// Importing the "YamlFrontMatter"
/// data structure for reading front matter.
use yaml_front_matter::YamlFrontMatter;

/// A data structure to store info
/// about a Markdown document.
pub struct MarkdownDocument {
    pub meta_data: HashMap<String, String>,
    pub content: String
}

/// Implementing methods for the "MarkdownDocument"
/// entity.
impl MarkdownDocument {

    /// A generic method
    /// to create new instances of this
    /// data structure.
    pub fn new(
        meta_data: &HashMap<String, String>, 
        content: &String) -> MarkdownDocument 
    {
        return MarkdownDocument {
            meta_data: meta_data.to_owned(),
            content: content.to_owned()
        };
    }

    /// Converts this data structure to a map that a Liquid template
    /// can render.
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();
        let meta_data: &HashMap<String, String> = &self.meta_data;
        for (k,v) in meta_data.into_iter() {
            result.insert(k.to_string(),v.to_string());
        }
        let content: &String = &self.content;
        result.insert(String::from("content"), content.to_owned());
        return result;
    }
}

/// Deserializes Markdown into a "MarkdownDocument" entity
/// or returns an error.
pub fn deserialize_md(
    md_string: &String
) -> Result<MarkdownDocument, MandyError> {
    let document = YamlFrontMatter::parse::<HashMap<String, String>>(
        md_string
    );
    match document {
        Ok(res) => {
            let result: MarkdownDocument = MarkdownDocument::new(
                &res.metadata,
                &to_html(&res.content)
            );
            return Ok(result);
        }, 
        Err(e) => {
            return Err::<MarkdownDocument, MandyError>(
                MandyError::new(
                    &e.to_string()
                )
            );
        }
    };
}

/// A structure to represent a single
/// Markdown document in a Mandy site.
/// This structure also contains information
/// on the file's path and parent directory.
pub struct MandyMDDocument {
    pub file: String,
    pub dir: String,
    pub ctx: HashMap<String, String>
}

/// Implementing generic methods
/// for the "MandyMDDocument" entity.
impl MandyMDDocument {

    /// Implementing an associated function
    /// to create a new instance of the
    /// "MandyMDDocument" entity.
    pub fn new(
        file: &String,
        dir: &String,
        ctx: &HashMap<String, String>
    ) -> MandyMDDocument {
        return MandyMDDocument{
            file: file.to_owned(),
            dir: dir.to_owned(),
            ctx: ctx.to_owned()            
        };
    }
}
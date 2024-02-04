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

/// Importing the "HashMap" data
/// structure for explicit type
/// declarations.
use std::collections::HashMap;

/// A data structure
/// that mimics the structure
/// of an RSS channel.
#[derive(Clone, Debug)]
pub struct Channel {
    pub title: String,
    pub description: String,
    pub link: String,
    pub items: Vec<Item>
}

/// Implementing generic
/// methods for the "Channel"
/// data structure.
impl Channel {

    /// A generic method
    /// to create a new instance
    /// of the "Channel" data structure.
    pub fn new(
        title: &String,
        description: &String,
        link: &String,
        items: &Vec<Item>
    ) -> Channel {
        Channel {
            title: title.to_owned(),
            description: description.to_owned(),
            link: link.to_owned(),
            items: items.to_owned()
        }
    }

    /// A method to create a representation of
    /// the "Channel" data structure in XML format.
    pub fn to_string(&self) -> String {
        let mut joined_items: Vec<String> = Vec::new();
        for item in &self.items {
            joined_items.push(item.to_string());
        }
        let items: String = joined_items.join("\n");
        format!(
            "<channel>\n<title>{}</title>\n<description>{}</description>\n<atom:link href=\"{}\" rel=\"self\" type=\"application/rss+xml\"/>\n{}\n</channel>",
            &self.title,
            &self.description,
            &self.link,
            items
        )
    }
}

/// A data structure
/// that mimics the way
/// an RSS item works.
#[derive(Clone, Debug)]
pub struct Item {
    pub title: String,
    pub description: String,
    pub pub_date: String,
    pub link: String,
}

/// Implementing generic
/// methods for the "Item"
/// data structure.
impl Item {

    /// A generic method
    /// to create a new instance
    /// of the "Item" data structure.
    pub fn new(
        title: &String,
        description: &String,
        pub_date: &String,
        link: &String,
    ) -> Item {
        Item {
            title: title.to_owned(),
            description: description.to_owned(),
            pub_date: pub_date.to_owned(),
            link: link.to_owned()
        }
    }

    /// A method to create a representation of
    /// the "Item" data structure in XML format.
    pub fn to_string(&self) -> String {
        format!(
            "<item>\n<title>{}</title>\n<description>{}</description>\n<pubDate>{}</pubDate>\n<link>{}</link>\n</item>",
            &self.title,
            &self.description,
            &self.pub_date,
            &self.link
        )
    }

}

/// A data structure
/// that holds a list
/// of all data channels
/// of an RSS feed.
pub struct RSSFeed{
    channels: Vec<Channel>,
}

/// Implementing generic
/// methods for the "RSSFeed"
/// data structure.
impl RSSFeed{

    /// A generic method
    /// to create a new instance
    /// of the "RSSFeed" data structure.
    pub fn new(channels: &Vec<Channel>) -> RSSFeed{
        RSSFeed{channels: channels.to_owned()}
    }

    /// A method to create a representation of
    /// the "RSSFeed" data structure in XML format.
    pub fn to_string(&self) -> String {
        let mut joined_channels: Vec<String> = Vec::new();
        for channel in &self.channels{
            joined_channels.push(
                format!("{}\n", channel.to_string())
            );
        }
        return format!("<rss version=\"2.0\">\n{}</rss>", joined_channels.join(""));
    }
}

/// This function creates a file called "rss.xml" at the root
/// of the built site and returns an error if this fails.
pub fn create_feed(
    dir: &String,
    site_contexts: &Vec<SiteContext>
) -> Result<(), MandyError>{
    let rss_path: String = format!("{}/dist/rss.xml", &dir);
    let mut channels: Vec<Channel> = Vec::new(); 
    if loop_content_is_same(site_contexts) && !(site_contexts.is_empty()){
        let site_context = &site_contexts[0].clone();
        match &site_context.loop_content{
            Some(content) => {
                    for (channel, items) in content {
                        let mut rss_items: Vec<Item> = Vec::new();
                        for item in items {
                            let rss_item: Item = Item::new(
                                &item["title"],
                                &item["description"],
                                &item["date"],
                                &format!("{}/{}", &site_context.baseurl, item["url"])
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
            {return Err::<(), MandyError>(MandyError::new(&e.to_string()));}
        }
        else {
            match create_file(&rss_path){
                Ok(_y) => {
                    match write_to_file(
                        &rss_path,
                        &RSSFeed::new(&channels).to_string()
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
    }
    Ok(())
}

/// This function checks all gathered pieces of site
/// data, checks the iterative content, removes any duplicates and
/// returns the iterative content in a format ready for generation of
/// an RSS feed.
pub fn loop_content_is_same(subject: &Vec<SiteContext>) -> bool {
    let mut result: bool = false;
    let mut loop_content_vec: Vec<Option<HashMap<String, Vec<HashMap<String, String>>>>> = Vec::new();
    for context in subject {
        loop_content_vec.push(context.clone().loop_content);
    }
    if loop_content_vec.is_empty(){result = true;}
    else {
        let first = loop_content_vec[0].clone();
        result = loop_content_vec.iter().all(|item| item == &first);
    }
    result
}
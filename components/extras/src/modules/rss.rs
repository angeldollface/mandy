/*
MANDY EXTRAS by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

use merrors::MandyError;
use coutils::write_to_file;
use coutils::create_file;
use coutils::file_is;
use context::SiteContext;
use std::collections::HashMap;
use serde_json::to_string_pretty;

#[derive(Clone, Debug)]
pub struct Channel {
    pub title: String,
    pub description: String,
    pub link: String,
    pub items: Vec<Item>
}

impl Channel {

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

#[derive(Clone, Debug)]
pub struct Item {
    pub title: String,
    pub description: String,
    pub pub_date: String,
    pub link: String,
}

impl Item {

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

pub fn create_feed(
    dir: &String,
    site_contexts: &Vec<SiteContext>
) -> Result<(), MandyError>{
    let rss_path: String = format!("{}/rss.xml", &dir);
    let mut loop_content_items: Vec<HashMap<String, Vec<HashMap<String, String>>>> = Vec::new();
    for ctx in site_contexts {
        match ctx.clone().loop_content{
            None => {},
            Some(content) => {
               loop_content_items.push(content);
            }
        }
    }
    get_channel_names(&loop_content_items);
    Ok(())
}

pub fn get_channel_names(subject: &Vec<HashMap<String, Vec<HashMap<String, String>>>>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut im_result: Vec<String> = Vec::new();
    for item in subject{
            let json_string: String = to_string_pretty(&item).unwrap();
            println!("{}", json_string);
    }
    return result;
}
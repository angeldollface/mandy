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

pub struct RSSFeed{
    channels: Vec<Channel>,
}

impl RSSFeed{
    pub fn new(channels: &Vec<Channel>) -> RSSFeed{
        RSSFeed{channels: channels.to_owned()}
    }
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
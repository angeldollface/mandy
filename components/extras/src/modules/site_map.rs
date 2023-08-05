/*
MANDY EXTRAS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// A data structure
/// to hold relevant information
/// for each "Url" in a
/// site's URLs.
#[derive(Clone)]
pub struct Url {
    loc: String,
    changefreq: String,
}

/// Implementing
/// methods for the "Url"
/// data structure.
impl Url {

    /// Convenience
    /// method to create
    /// a new instance
    /// of the "Url"
    /// data structure.
    pub fn new(
        loc: &String,
        changefreq: &String
    ) -> Url {
        return Url {
            loc: loc.to_owned(),
            changefreq: changefreq.to_owned()
        };
    }

    /// Creates an XML representation
    /// of the information in the data
    /// structure.
    pub fn to_xml(&self) -> String {
        return format!(
            "<url><loc>{}</loc>\n<changefreq>{}</changefreq>\n</url>",
            &self.loc,
            &self.changefreq
        );
    }
}

/// A data structure
/// to hold a set of
/// URLs.
#[derive(Clone)]
pub struct UrlSet {
    urls: Vec<Url>
}

/// Implementing
/// methods for the "UrlSet"
/// data structure.
impl UrlSet {

    /// Convenience
    /// method to create
    /// a new instance
    /// of the "UrlSet"
    /// data structure.
    pub fn new(
        urls: &Vec<Url>
    ) -> UrlSet {
        return UrlSet {
            urls: urls.to_owned()
        };
    }

    /// Creates an XML representation
    /// of the information in the data
    /// structure.
    pub fn to_xml(&self) -> String {
        let mut string_vec: Vec<String> = Vec::new();
        for url in &self.urls {
            string_vec.push(url.to_xml());
        }
        let schema_url: String = String::from("http://www.sitemaps.org/schemas/sitemap/0.9");
        let schema_ref: String = format!("xmlns=\"{}\"", schema_url);
        let schema_image_url: String = String::from("http://www.google.com/schemas/sitemap-image/1.1");
        let schema_image_ref: String = format!("xmlns:image=\"{}\"", schema_image_url);
        let result: String = format!("<urlset {} {}>\n{}\n</urlset>", schema_ref, schema_image_ref, string_vec.join("\n"));
        return result;
    }
}

/// The method to create the
/// string for "sitemap.xml".
pub fn create_site_map(
    urls: &Vec<String>,
    freq: &String,
    baseurl: &String, 
    top_level_domain: &String
) -> String {
    let mut site_map_urls: Vec<Url> = Vec::new();
    for url in urls {
        let full_url: String = format!(
            "{}{}{}",
            top_level_domain, 
            baseurl, 
            url
        );
        let new_url: Url = Url::new(
            &full_url,
            freq
        );
        site_map_urls.push(new_url);
    }
    let result: UrlSet = UrlSet::new(&site_map_urls);
    return result.to_xml();
}
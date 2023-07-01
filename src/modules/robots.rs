/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// The method to create the
/// string for "robots.txt". Information is drawn
/// from the site's "baseurl" and top-level domain.
pub fn create_robots_txt(top_level_domain: &String, baseurl: &String) -> String {
    return format!(
        "User-agent: *\nAllow: /\nSitemap: {}{}/sitemap.xml", 
        top_level_domain, 
        baseurl
    );
} 
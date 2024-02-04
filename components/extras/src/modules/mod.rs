/*
MANDY EXTRAS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Exporting the module
/// that handles the generation
/// of an XML file for an RSS
/// feed of iterative content.
pub mod rss;

pub mod api;

/// Exporting the module
/// that creates files for
/// Deno Deploy.
pub mod deno;

/// Exporting the module that
/// compiles SCSS files.
pub mod sass;

/// Exporting the module
/// that generates 
/// a "robots.txt" file.
pub mod robots;

/// Exporting the module
/// that helps to log
/// the build of a Mandy
/// project.
pub mod logging;

/// Exporting the module
/// that creates files for
/// SEO crawlers.
pub mod crawlers;

/// Exporting the module
/// that generates 
/// a "sitemap.xml" file.
pub mod site_map;

/// Exporting the module
/// that generates 
/// a build info file.
pub mod build_meta;

pub mod categories;
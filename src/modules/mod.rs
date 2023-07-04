/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Exporting the CLI
/// module.
pub mod cli;

/// Exporting the 
/// module for getting
/// vars about Mandy herself.
pub mod vars;

/// Exporting the module to handle
/// site data.
pub mod data;

/// Exporting the module 
/// to compile SASS
/// files.
pub mod sass;

/// Exporting the module that
/// contains some utility functions.
pub mod utils;

/// Exporting Mandy's error-handling
/// module.
pub mod errors;

/// Exporting the module
/// to create the string 
/// for the "robots.txt" 
/// file.
pub mod robots;

/// Exporting the module
/// to handle configuration
/// reading and serializing.
pub mod config;

/// Exporting the module
/// to handle Markdown
/// reading and serializing.
pub mod finder;

/// Exporting the module
/// to handle Liquid templating.
pub mod liquid;

/// Exporting the module
/// for server-related utility
/// functions.
pub mod server;

/// Exporting the module
/// for detecting in which
/// environment a Mandy site
/// is being compiled in.
pub mod compenv;

/// Exporting the module
/// that compiles and builds
/// a Mandy site.
pub mod compile;

/// Exporting the module
/// to create all needed files
/// for search-engine crawling
/// bots for Mandy sites and SEO.
pub mod crawlers;

/// Exporting the module
/// that handles deserializing
/// Markdown data.
pub mod markdown;

/// Exporting the module
/// that handles scaffolding
/// a new Mandy site.
pub mod scaffold;

/// Exporting the module
/// that holds the main
/// data context data
/// structure.
pub mod context;

/// Exporting the module
/// that handles retrieving
/// the data in JSON format
/// of a Mandy site.
pub mod get_data;

/// Exporting the module
/// to create the string 
/// for the "sitemap.xml" 
/// file.
pub mod site_map;

/// Exporting the module
/// that handles retrieving
/// the data in Markdown format
/// of a Mandy site.
pub mod get_pages;

/// Exporting the module
/// that parses an arrow set
/// string into a Rust data-
/// structure.
pub mod arrow_set;

/// Exporting the module
/// that generates a meta data
/// file on a build of a Mandy
/// site.
pub mod build_meta;

/// Exporting the module
/// that handles retrieving
/// the data context
/// of a Mandy site.
pub mod get_context;

/// Exporting the module
/// to retrieve partial
/// Liquid templates.
pub mod get_partials;

/// Exporting the module
/// that builds and renders
/// a single data context.
pub mod build_context;

/// Exporting the module
/// that retrieves the contexts
/// for content the user would
/// like to loop over.
pub mod get_loop_content;
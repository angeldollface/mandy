/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the CLI
/// module.
pub use modules::cli::*;

/// Re-exporting the module to handle
/// site data.
pub use modules::data::*;

/// Re-exporting the module 
/// to compile SASS
/// files.
pub use modules::sass::*;

/// Re-exporting the module that
/// contains some utility functions.
pub use modules::utils::*;

/// Re-exporting Mandy's error-handling
/// module.
pub use modules::errors::*;

/// Re-exporting the module
/// to handle configuration
/// reading and serializing.
pub use modules::config::*;

/// Re-exporting the module
/// to handle Markdown
/// reading and serializing.
pub use modules::finder::*;

/// Re-exporting the module
/// to handle Liquid templating.
pub use modules::liquid::*;

/// Re-exporting the module
/// for server-related utility
/// functions.
pub use modules::server::*;

/// Re-exporting the module
/// for detecting in which
/// environment a Mandy site
/// is being compiled in.
pub use modules::compenv::*;

/// Re-exporting the module
/// that compiles and builds
/// a Mandy site.
pub use modules::compile::*;

/// Re-exporting the module
/// that handles deserializing
/// Markdown data.
pub use modules::markdown::*;

/// Re-exporting the module
/// that handles scaffolding
/// a new Mandy site.
pub use modules::scaffold::*;

/// Re-exporting the module
/// that holds the main
/// data context data
/// structure.
pub use modules::context::*;

/// Re-exporting the module
/// that handles retrieving
/// the data in JSON format
/// of a Mandy site.
pub use modules::get_data::*;

/// Re-exporting the module
/// that handles retrieving
/// the data in Markdown format
/// of a Mandy site.
pub use modules::get_pages::*;

/// Re-exporting the module
/// that parses an arrow set
/// string into a Rust data-
/// structure.
pub use modules::arrow_set::*;

/// Re-exporting the module
/// that handles retrieving
/// the data context
/// of a Mandy site.
pub use modules::get_context::*;

/// Re-exporting the module
/// that builds and renders
/// a single data context.
pub use modules::build_context::*;

/// Re-exporting the module
/// that retrieves the contexts
/// for content the user would
/// like to loop over.
pub use modules::get_loop_content::*;
/*
MANDY DATA by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// directory as a 
/// module.
pub mod modules;

/// Re-exporting the 
/// module that handles
/// site data in a Mandy
/// project.
pub use modules::get_data::*;

/// Re-exporting the 
/// module that handles
/// the data of Markdown files
/// in a Mandy project.
pub use modules::get_pages::*;

/// Re-exporting the module
/// that attempts to retrieve 
/// all relevant
/// data of a Mandy project.
pub use modules::get_context::*;

/// Re-exporting the module
/// that handles Liquid
/// partial templates.
pub use modules::get_partials::*;

/// Re-exporting the module
/// that handles iterative
/// content in a Mandy site.
pub use modules::get_loop_content::*;
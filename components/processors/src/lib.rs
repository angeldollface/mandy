/*
MANDY PROCESSORS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// to handle data files in
/// Mandy sites.
pub use modules::data::*;

/// Re-exporting the module
/// to read the configuration
/// settings of a Mandy site.
pub use modules::config::*;

/// Re-exporting the module
/// to render Liquid templates in
/// Mandy sites.
pub use modules::liquid::*;

/// Re-exporting the module
/// to handle Markdown files in
/// Mandy sites.
pub use modules::markdown::*;

/// Re-exporting the module
/// to handle arrow expressions in
/// Mandy sites.
pub use modules::arrow_set::*;
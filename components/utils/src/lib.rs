/*
MANDY UTILS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the
/// module that contains
/// utility methods.
pub use modules::utils::*;

/// Re-exporting the module
/// that contains Mandy's
/// site server.
pub use modules::server::*;

/// Re-exporting the module
/// that finds Markdown files.
pub use modules::finder::*;

/// Re-exporting the module that
/// tries to detect the compilation
/// environment.
pub use modules::compenv::*;

/// Re-exporting the module
/// that scaffolds a new Mandy
/// site from an existing one.
pub use modules::scaffold::*;
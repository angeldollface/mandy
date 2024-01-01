/*
DOLLMARKUP by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting all APIs from
/// the AST module.
pub use modules::ast::*;

/// Re-exporting all APIs from
/// the lexer.
pub use modules::lexer::*;

/// Re-exporting all APIs from
/// the processor module.
pub use modules::processor::*;
/*
MANDY COMPILER by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting Mandy's compiler
/// module.
pub use modules::compile::*;

/// Re-exporting Mandy's context
/// builder module.
pub use modules::build_context::*;
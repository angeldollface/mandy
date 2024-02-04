/*
MANDY EXTRAS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// that handles the generation
/// of an XML file for an RSS
/// feed of iterative content.
pub use modules::rss::*;

/// Re-exporting the module
/// that handles the generation
/// of a JSON API containing
/// all iterative content
/// users have made.
pub use modules::api::*;

/// Re-exporting the module
/// that creates files for
/// Deno Deploy.
pub use modules::deno::*;

/// Re-exporting the module that
/// compiles SCSS files.
pub use modules::sass::*;

/// Re-exporting the module
/// that generates 
/// a "robots.txt" file.
pub use modules::robots::*;

/// Re-exporting the module
/// that helps to log
/// the build of a Mandy
/// project.
pub use modules::logging::*;

/// Re-exporting the module
/// that creates files for
/// SEO crawlers.
pub use modules::crawlers::*;

/// Re-exporting the module
/// that generates 
/// a "robots.txt" file.
pub use modules::site_map::*;

/// Re-exporting the module
/// that generates 
/// a build info file.
pub use modules::build_meta::*;

/// Re-exporting the module
/// that handles the ordering
/// of iterative content
/// into categories.
pub use modules::categories::*;
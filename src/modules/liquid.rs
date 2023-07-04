/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the 
/// "object" macro
/// from the "liquid"
/// crate.
use liquid::object;

/// Importing Rust's
/// standard "Debug"
/// trait.
use std::fmt::Debug;

/// Importing Liquid's
/// standard "ValueView"
/// trait.
use liquid::ValueView;

/// Importing Liquid's
/// standard "ObjectView"
/// trait.
use liquid::ObjectView;

/// Importing Liquid's
/// standard "ParserBuilder"
/// entity.
use liquid::ParserBuilder;

/// Importing Mandy's error
/// struct.
use super::errors::MandyError;

/// Importing Rust's standard
/// "HashMap" API.
use std::collections::HashMap;

/// Importing the "EagerCompiler"
/// structure from the "liquid" crate.
use liquid::partials::EagerCompiler;

// Importing the "InMemorySource"
/// structure from the "liquid" crate.
use liquid::partials::InMemorySource;

/// Renders a string written in Liquid with a context that
/// implements T's traits and returns a string or an error.
pub fn render_template<T: ObjectView + ValueView + Debug>(
    liquid_string: &String, 
    context: &T,
    partials: &Option<HashMap<String, String>>
) -> Result<String, MandyError> {
    let mut result: String = String::from("");
    match partials {
        Some(map) => {
            println!("{:?}", &map);
            type Partials =  EagerCompiler<InMemorySource>;
            let mut partial_source = Partials::empty();
            for (k,v) in map.into_iter() {
                partial_source.add(k,v);
            }
            let mut liquid_parser = match ParserBuilder::with_stdlib().partials(partial_source).build() {
                Ok(liquid_parser) => liquid_parser,
                Err(e) => {
                    println!("With partials.");
                    return Err::<String, MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );
                }
            };
            let mut liquid_template = match liquid_parser.parse(&liquid_string){
                Ok(liquid_template) => liquid_template,
                Err(e) => {
                    println!("Ha!");
                    return Err::<String, MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );
                }
            };
            let globals = object!(context);
            let mut output: String = match liquid_template.render(&globals){
                Ok(output) => output,
                Err(render_error) => {
                    return Err::<String, MandyError>(
                        MandyError::new(
                            &render_error.to_string()
                        )
                    );
                }
            };
            result = output;
        },
        None => {
            let mut liquid_parser = match ParserBuilder::with_stdlib().build() {
                Ok(liquid_parser) => liquid_parser,
                Err(e) => {
                    println!("Without partials.");
                    return Err::<String, MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );
                }
            };
            let mut liquid_template = match liquid_parser.parse(&liquid_string){
                Ok(liquid_template) => liquid_template,
                Err(e) => {
                    println!("Ha!");
                    return Err::<String, MandyError>(
                        MandyError::new(
                            &e.to_string()
                        )
                    );
                }
            };
            let globals = object!(context);
            let mut output: String = match liquid_template.render(&globals){
                Ok(output) => output,
                Err(render_error) => {
                    return Err::<String, MandyError>(
                        MandyError::new(
                            &render_error.to_string()
                        )
                    );
                }
            };
            result = output;
        }
    };
    return Ok(result);
}
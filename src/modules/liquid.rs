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

/// Renders a string written in Liquid with a context that
/// implements T's traits and returns a string or an error.
pub fn render_template<T: ObjectView + ValueView + Debug>(
    liquid_string: &String, 
    context: &T
) -> Result<String, MandyError> {
    let liquid_string = ParserBuilder::with_stdlib().build().unwrap().parse(
        &liquid_string
    );
    match liquid_string {
        Ok(x) => {
            let globals = object!(context);
            let output = x.render(&globals);
            match output {
                Ok(y) => {
                    return Ok(y);
                },
                Err(render_error) => {
                    return Err::<String, MandyError>(
                        MandyError::new(
                            &render_error.to_string()
                        )
                    );
                }
            }
        },
        Err(error) => {
            return Err::<String, MandyError>(
                MandyError::new(
                    &error.to_string()
                )
            );
        }
    }
}
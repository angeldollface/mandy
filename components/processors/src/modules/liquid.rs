/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the 
/// "object" macro
/// from the "liquid"
/// crate.
use liquid::object;

/// Importing Liquid's
/// standard "ParserBuilder"
/// entity.
use liquid::ParserBuilder;

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// Importing Rust's standard
/// "HashMap" API.
use std::collections::HashMap;

/// Importing the structure representing
/// a Mandy site's context.
use context::SiteContext;

/// Importing the "EagerCompiler"
/// structure from the "liquid" crate.
use liquid::partials::EagerCompiler;

/// Importing the "InMemorySource"
/// structure from the "liquid" crate.
use liquid::partials::InMemorySource;

/// Renders a string written in Liquid with a context 
/// and returns a string or an error.
pub fn render_template_no_partials(
    liquid_string: &String,
    context: &SiteContext
) -> Result<String, MandyError> {
    let mut result: String = String::from("");
    let mut liquid_parser = match ParserBuilder::with_stdlib().build() {
        Ok(liquid_parser) => liquid_parser,
        Err(e) => {return Err::<String, MandyError>(MandyError::new(&e.to_string()));}
    };
    let mut liquid_template = match liquid_parser.parse(&liquid_string){
        Ok(liquid_template) => liquid_template,
        Err(e) => {
        return Err::<String, MandyError>(MandyError::new(&e.to_string()));}
    };
    let globals = object!(context);
    let mut output: String = match liquid_template.render(&globals){
        Ok(output) => output,
        Err(render_error) => {return Err::<String, MandyError>(MandyError::new(&render_error.to_string()));}
    };
    return Ok(output);
}

/// Renders a string written in Liquid with a context and
/// partials and returns a string or an error.
pub fn render_template_partials(
    liquid_string: &String,
    context: &SiteContext,
    partials: &HashMap<String, String>
) -> Result<String, MandyError>{
    type Partials =  EagerCompiler<InMemorySource>;
    let mut partial_source = Partials::empty();
    for (k,v) in partials.into_iter() {
        partial_source.add(k,v);
    }
    let mut liquid_parser = match ParserBuilder::with_stdlib().partials(partial_source).build() {
        Ok(liquid_parser) => liquid_parser,
        Err(e) => {return Err::<String, MandyError>(MandyError::new(&e.to_string()));}
    };
    let mut liquid_template = match liquid_parser.parse(&liquid_string){
        Ok(liquid_template) => liquid_template,
        Err(e) => {return Err::<String, MandyError>(MandyError::new(&e.to_string()));}
    };
    let globals = object!(context);
    let mut output: String = match liquid_template.render(&globals){
        Ok(output) => output,
        Err(render_error) => {return Err::<String, MandyError>(MandyError::new(&render_error.to_string()));}
    };
    return Ok(output);
}

/// Renders a string written in Liquid with a context
/// and returns a string or an error.
pub fn render_template(
    liquid_string: &String, 
    context: &SiteContext,
    partials: &Option<HashMap<String, String>>
) -> Result<String, MandyError> {
    let mut result: String = String::from("");
    match partials {
        Some(map) => {  
            let mut output: String = match render_template_partials(liquid_string, context, map){
                Ok(output) => output,
                Err(render_error) => {return Err::<String, MandyError>(MandyError::new(&render_error.to_string()));}
            };  
            result = output;        
        },
        None => {
            let mut output: String = match render_template_no_partials(liquid_string, context){
                Ok(output) => output,
                Err(render_error) => {return Err::<String, MandyError>(MandyError::new(&render_error.to_string()));}
            };
            result = output;
        }
    };
    return Ok(result);
}
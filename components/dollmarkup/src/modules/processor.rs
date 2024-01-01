/*
DOLLMARKUP by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "Deserialize"
/// trait to use it in the "from_str"
/// method.
use serde::Deserialize;

/// Importing Mandy's error
/// structure to catch and return
/// errors.
use merrors::MandyError;

use super::lexer::Token;

use super::ast::parse_tokens;

use super::lexer::tokenize;

/// Takes in a Dollmarkup
/// string, returns an instance
/// of the cast type.
pub fn from_str<T: Deserialize>(subject: &String) -> Result<T, MandyError>{
    let token_stream: Vec<Token> = match tokenize(subject){
        Ok(token_stream) => token_stream,
        Err(e) => {
        return Err::<T, MandyError>(
            MandyError::new(&e.to_string())
        );
        }
    };
    let stmts: Vec<DollMarkupEntity> = parse_tokens(&token_stream){
        Ok(stmts) => stmts,
        Err(e) => {
            return Err::<T, MandyError>(
                MandyError::new(&e.to_string())
            );
        }
    }

}
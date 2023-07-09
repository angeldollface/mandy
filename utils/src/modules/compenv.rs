/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the function
/// to get environment variable
/// info from the standard library.
use std::env::var;

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// An enum with two variants
/// to enumerate the possible
/// environments in which a 
/// Mandy website is compiled.
#[derive(PartialEq, Clone)]
pub enum Environment {
    Production,
    Development
}

/// Attempts to determine the environment in which a Mandy
/// site is being compiled. Depending on whether the "$MANDY_ENV"
/// environment variable is either set to "production" or "development",
/// the functions returns a variant of the "Environment" enum.
pub fn detect_env() -> Result<Environment, MandyError> {
    let var_op = var("MANDY_ENV");
    match var_op {
        Ok(x) => {
            if x == "production" {
               return Ok(Environment::Production);
            }
            else if x == "development" {
                return Ok(Environment::Development);
            }
            else {
                let err_msg: &String = &format!("No valid value set for \"$MANDY_ENV\".");
                return Err::<Environment, MandyError>(
                    MandyError::new(
                        &err_msg.to_string()
                    )
                );
            }
        },
        Err(e) => {
            return Err::<Environment, MandyError>(
                MandyError::new(
                    &e.to_string()
                )
            );
        }
    };
}
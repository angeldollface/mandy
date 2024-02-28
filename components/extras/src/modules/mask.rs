/*
MANDY EXTRAS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

use std::env::var;
use merrors::MandyError;

pub fn parse_mask(mask_expression: &String) -> Result<String, MandyError>{
    let mut mask_expression_chars: Vec<char> = mask_expression.chars().collect();
    let last_index: usize = mask_expression_chars.len();
    let last_char: char = mask_expression_chars[last_index];
    if mask_expression_chars[0] == '%' &&
        last_char == '%'
    {
        mask_expression_chars.remove(0);
        mask_expression_chars.remove(last_index);
        let collected: String = mask_expression_chars.iter().collect::<String>();
        Ok(collected)
    }
    else {
        let e: String = format!("Error parsing expression \"{}\"!", &mask_expression);
        Err::<String, MandyError>(MandyError::new(&e.to_string()))
    }
}

pub fn unmask_variable(mask_expression: &String) -> Result<String, MandyError>{
    let env_var: String = match parse_mask(mask_expression){
        Ok(unmasked) => unmasked,
        Err(e) => return Err::<String, MandyError>(MandyError::new(&e.to_string()))
    };
    let unmasked: String = match var(env_var){
        Ok(unmasked) => unmasked,
        Err(e) => return Err::<String, MandyError>(MandyError::new(&e.to_string()))
    };
    Ok(unmasked)
}
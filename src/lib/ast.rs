/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

mod lexer;
mod grammar;

use super::grammar::Grammar;
use super::lexer::TokenStream;

pub struct Tree {

}

impl Tree {
    pub fn to_string(&self) -> String {
        let mut result: String = String::from("");
        return result;
    }
}

pub fn generate_parse_tree(tokens: &TokenStream, grammar: &Grammar) -> Tree {
    return Tree{};
}
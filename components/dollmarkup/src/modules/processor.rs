/*
DOLLMARKUP by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing Mandy's error
/// structure to catch and return
/// errors.
use merrors::MandyError;

/// Importing the "Token"
/// data structure to
/// cast everything strictly.
use super::lexer::Token;

/// Importing the method
/// to tokenize input
/// string.
use super::lexer::tokenize;

/// Importing the method
/// to parse a token sequence
/// into a set of statements.
use super::ast::parse_tokens;
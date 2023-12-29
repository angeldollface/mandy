/*
VENTO OXIDE by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "Token"
/// structure to capture info
/// about tokens.
use super::lexer::Token;

/// Importing the "Token"
/// structure to tokenize input.
use super::lexer::tokenize;

/// Importing the "Token"
/// structure to capture info
/// about the type of a token.
use super::lexer::TokenType;

/// Tests input with a tag
/// and a user string.
#[test]
pub fn test_h1() {
    let code: String = "<h1>{{ message }}</h1>".to_string();
    let res: Vec<Token> = tokenize(&code).unwrap();
    let mut res_vec: Vec<Token> = Vec::new();
    res_vec.push(
        Token::new(&"<h1>", &TokenType::Tag, &3)
    );
    res_vec.push(
        Token::new(&"{{ message }}", &TokenType::UserString, &16)
    );
    res_vec.push(
        Token::new(&"</h1>", &TokenType::Tag, &21)
    );
    assert_eq!(res, res_vec);
}

/// Tests input with a comment.
#[test]
pub fn test_comment() {
    let code: String = "{{# message #}}".to_string();
    let res: Vec<Token> = tokenize(&code).unwrap();
    let mut res_vec: Vec<Token> = Vec::new();
    res_vec.push(
        Token::new(&"{{# message #}}",&TokenType::Comment,&14)
    );
    assert_eq!(res, res_vec);
}

/// Tests input with a comment.
#[test]
pub fn test_invalid_input() {
    let code: String = "{{# message ".to_string();
    assert_eq!(tokenize(&code).is_ok(), false);
}
/*
DOLLMARKUP by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Imports the "Deserialize"
/// trait to use in tests.
use serde::Deserialize;

/// Importing the "Token"
/// structure to capture info
/// about tokens.
use super::lexer::Token;

/// Importing the enum
/// that describes statement
/// types.
use super::ast::StmtType;

/// Importing the "Token"
/// structure to tokenize input.
use super::lexer::tokenize;

/// Importing the "Token"
/// structure to capture info
/// about the type of a token.
use super::lexer::TokenType;

/// Importing the method parse tokens
/// into statements.
use super::ast::parse_tokens;

/// Importing the structure to store info
/// about parsed tokens.
use super::ast::DollMarkupEntity;

/// Importing the "from_str"
/// structure to capture info
/// from Dollmarkup into a Rust
/// data structure.
//use super::processor::from_str;

/// Tests input with a string
/// assigned to a string.
#[test]
pub fn test_string() {
    let code: String = "\"name\"=>\"mandy\"".to_string();
    let res: Vec<Token> = tokenize(&code).unwrap();
    let mut res_vec: Vec<Token> = Vec::new();
    res_vec.push(
        Token::new(&"\"name\"", &TokenType::UserString, &5)
    );
    res_vec.push(
        Token::new(&"=>", &TokenType::Assign, &7)
    );
    res_vec.push(
        Token::new(&"\"mandy\"", &TokenType::UserString, &14)
    );
    assert_eq!(res, res_vec);
}

/// Tests input with a comment.
#[test]
pub fn test_comment() {
    let code: String = "// A random comment.\n".to_string();
    let res: Vec<Token> = tokenize(&code).unwrap();
    let mut res_vec: Vec<Token> = Vec::new();
    res_vec.push(
        Token::new(&"// A random comment.\n",&TokenType::Comment,&20)
    );
    assert_eq!(res, res_vec);
}

/// Tests input with a boolean "true".
#[test]
pub fn test_bool_true() {
    let code: String = "\"name\"=>true".to_string();
    let res: Vec<Token> = tokenize(&code).unwrap();
    let mut res_vec: Vec<Token> = Vec::new();
    res_vec.push(
        Token::new(&"\"name\"",&TokenType::UserString,&5)
    );
    res_vec.push(
        Token::new(&"=>",&TokenType::Assign,&7)
    );
    res_vec.push(
        Token::new(&"true",&TokenType::BooleanTrue,&11)
    );
    assert_eq!(res, res_vec);
}
/// Tests input with a boolean "false".
#[test]
pub fn test_bool_false() {
    let code: String = "\"name\"=>false".to_string();
    let res: Vec<Token> = tokenize(&code).unwrap();
    let mut res_vec: Vec<Token> = Vec::new();
    res_vec.push(
        Token::new(&"\"name\"",&TokenType::UserString,&5)
    );
    res_vec.push(
        Token::new(&"=>",&TokenType::Assign,&7)
    );
    res_vec.push(
        Token::new(&"false",&TokenType::BooleanFalse,&12)
    );
    assert_eq!(res, res_vec);
}

/// Tests input with a section identifier.
#[test]
pub fn test_section_id() {
    let code: String = ":section \"name\"".to_string();
    let res: Vec<Token> = tokenize(&code).unwrap();
    let mut res_vec: Vec<Token> = Vec::new();
    res_vec.push(
        Token::new(&":section ",&TokenType::SectionDef,&8)
    );
    res_vec.push(
        Token::new(&"\"name\"",&TokenType::UserString,&14)
    );
    assert_eq!(res, res_vec);
}

#[test]
pub fn test_number(){
    let code: String = "\"name\"=>(564)".to_string();
    let res: Vec<Token> = tokenize(&code).unwrap();
    let mut res_vec: Vec<Token> = Vec::new();
    res_vec.push(
        Token::new(&"\"name\"",&TokenType::UserString,&5)
    );
    res_vec.push(
        Token::new(&"=>",&TokenType::Assign,&7)
    );
    res_vec.push(
        Token::new(&"(564)",&TokenType::Number,&12)
    );
    assert_eq!(res, res_vec);
}

/// Tests input with invalid input.
#[test]
pub fn test_invalid_input() {
    let code: String = "\"name".to_string();
    assert_eq!(tokenize(&code).is_ok(), false);
}

#[test]
pub fn test_ast(){
    let code: String = "\"name\"=>(564)".to_string();
    let tokenized: Vec<Token> = tokenize(&code).unwrap();
    let parsed: Vec<DollMarkupEntity> = parse_tokens(&tokenized).unwrap();
    let expected: Vec<DollMarkupEntity> = vec![DollMarkupEntity::new(&StmtType::StringNumberStmt, "\"name\"", "(564)")];
    assert_eq!(parsed, expected);
}
/*
DOLLMARKUP by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

use super::lexer::Token;
use merrors::MandyError;
use super::lexer::TokenType;

#[derive(Clone, Debug,PartialEq)]
pub enum StringBoolFalseStmt{
    UserString,
    BooleanFalse
}

#[derive(Clone, Debug,PartialEq)]
pub enum StringBoolTrueStmt{
    UserString,
    BooleanTrue
}

#[derive(Clone, Debug,PartialEq)]
pub enum BoolFalseStringStmt{
    BooleanFalse,
    UserString
}

#[derive(Clone, Debug,PartialEq)]
pub enum BoolTrueStringStmt{
    BooleanTrue,
    UserString
}

#[derive(Clone, Debug,PartialEq)]
pub enum StringNumberStmt{
    UserString,
    Number
}

#[derive(Clone, Debug,PartialEq)]
pub enum NumberStringStmt{
    Number,
    UserString
}

#[derive(Clone, Debug,PartialEq)]
pub enum NumberBooleanTrueStmt{
    Number,
    BooleanTrue
}

#[derive(Clone, Debug,PartialEq)]
pub enum NumberBooleanFalseStmt{
    Number,
    BooleanFalse
}

#[derive(Clone, Debug,PartialEq)]
pub enum SectionStmt {
    SectionDef,
    UserString
}

#[derive(Clone, Debug,PartialEq)]
pub enum StmtType {
    StringBoolFalseStmt,
    StringBoolTrueStmt,
    BoolFalseStringStmt,
    BoolTrueStringStmt,
    StringNumberStmt,
    NumberStringStmt,
    NumberBooleanTrueStmt,
    NumberBooleanFalseStmt,
    SectionStmt
}

#[derive(Clone, Debug,PartialEq)]
pub struct DollMarkupEntity {
    pub stmt_type: StmtType,
    pub key: String,
    pub value: String
}

impl DollMarkupEntity {
    pub fn new(
        stmt_type: &StmtType,
        key: &String,
        value: &String
    ) -> DollMarkupEntity {
        DollMarkupEntity{
            stmt_type: stmt_type.to_owned(),
            key: key.to_owned(),
            value: value.to_owned()
        }
    }
}

pub fn parse_tokens(
    tokens: &Vec<Token>
) -> Result<Vec<DollMarkupEntity>, MandyError> {
    let mut result: Vec<DollMarkupEntity> = Vec::new();
    let mut token_pool: Vec<Token> = Vec::new();
    let tokens_length: usize = tokens.len();
    let mut counter: usize = 0;
    while counter < tokens_length{
        
        counter = counter + 1;
    }
    Ok(result)
}
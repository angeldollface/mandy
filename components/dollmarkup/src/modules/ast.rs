/*
DOLLMARKUP by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

use super::lexer::Token;
use super::lexer::TokenType;
use merrors::MandyError;

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
        key: &str,
        value: &str
    ) -> DollMarkupEntity {
        DollMarkupEntity{
            stmt_type: stmt_type.to_owned(),
            key: key.to_string(),
            value: value.to_string()
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
        token_pool.push(tokens[counter].clone());
        if is_string_bool_true_stmt(&token_pool){
            result.push(
                DollMarkupEntity::new(
                    &StmtType::StringBoolTrueStmt,
                    &token_pool[0].value,
                    &token_pool[2].value
                )
            );
            token_pool = Vec::new();
        }
        if is_string_bool_false_stmt(&token_pool){
            result.push(
                DollMarkupEntity::new(
                    &StmtType::StringBoolFalseStmt,
                    &token_pool[0].value,
                    &token_pool[2].value
                )
            );
            token_pool = Vec::new();
        }
        if is_string_number_stmt(&token_pool){
            result.push(
                DollMarkupEntity::new(
                    &StmtType::StringNumberStmt,
                    &token_pool[0].value,
                    &token_pool[2].value
                )
            );
            token_pool = Vec::new();
        }
        if is_number_string_stmt(&token_pool){
            result.push(
                DollMarkupEntity::new(
                    &StmtType::NumberStringStmt,
                    &token_pool[0].value,
                    &token_pool[2].value
                )
            );
            token_pool = Vec::new();
        }
        if is_number_bool_true_stmt(&token_pool){
            result.push(
                DollMarkupEntity::new(
                    &StmtType::NumberBooleanTrueStmt,
                    &token_pool[0].value,
                    &token_pool[2].value
                )
            );
            token_pool = Vec::new();
        }
        if is_number_bool_false_stmt(&token_pool){
            result.push(
                DollMarkupEntity::new(
                    &StmtType::NumberBooleanFalseStmt,
                    &token_pool[0].value,
                    &token_pool[2].value
                )
            );
            token_pool = Vec::new();
        }
        if is_section_stmt(&token_pool){
            result.push(
                DollMarkupEntity::new(
                    &StmtType::SectionStmt,
                    &token_pool[0].value,
                    &token_pool[2].value
                )
            );
            token_pool = Vec::new();
        }
        else {}
        counter = counter + 1;
    }
    if result.is_empty(){
        let e: String = format!("No valid token sequences detected.");
        return Err::<Vec<DollMarkupEntity>, MandyError>(
            MandyError::new(&e.to_string())
        );
    }
    else {}
    Ok(result)
}

pub fn is_string_bool_true_stmt(seq: &Vec<Token>) -> bool {
    let mut result: bool = false;
    if seq.len() == 3{
        if seq[0].token_type == TokenType::UserString &&
            seq[1].token_type == TokenType::Assign && 
            seq[2].token_type == TokenType::BooleanTrue
        {
            result = true;
        }
        else {}
    }
    else {}
    result
}

pub fn is_string_bool_false_stmt(seq: &Vec<Token>) -> bool {
    let mut result: bool = false;
    if seq.len() == 3{
        if seq[0].token_type == TokenType::UserString &&
            seq[1].token_type == TokenType::Assign && 
            seq[2].token_type == TokenType::BooleanFalse
        {
            result = true;
        }
        else {}
    }
    else {}
    result
}

pub fn is_string_number_stmt(seq: &Vec<Token>) -> bool {
    let mut result: bool = false;
    if seq.len() == 3{
        if seq[0].token_type == TokenType::UserString &&
            seq[1].token_type == TokenType::Assign &&
            seq[2].token_type == TokenType::Number
        {
            result = true;
        }
        else {}
    }
    else {}
    result
}

pub fn is_number_string_stmt(seq: &Vec<Token>) -> bool {
    let mut result: bool = false;
    if seq.len() == 3{
        if seq[0].token_type == TokenType::Number &&
            seq[1].token_type == TokenType::Assign && 
            seq[2].token_type == TokenType::UserString
        {
            result = true;
        }
        else {}
    }
    else {}
    result
}

pub fn is_number_bool_true_stmt(seq: &Vec<Token>) -> bool {
    let mut result: bool = false;
    if seq.len() == 3{
        if seq[0].token_type == TokenType::Number &&
            seq[1].token_type == TokenType::Assign && 
            seq[2].token_type == TokenType::BooleanTrue
        {
            result = true;
        }
        else {}
    }
    else {}
    result
}

pub fn is_number_bool_false_stmt(seq: &Vec<Token>) -> bool {
    let mut result: bool = false;
    if seq.len() == 3{
        if seq[0].token_type == TokenType::Number &&
            seq[1].token_type == TokenType::Assign && 
            seq[2].token_type == TokenType::BooleanFalse
        {
            result = true;
        }
        else {}
    }
    else {}
    result
}

pub fn is_section_stmt(seq: &Vec<Token>) -> bool {
    let mut result: bool = false;
    if seq.len() == 2{
        if seq[0].token_type == TokenType::SectionDef &&
            seq[1].token_type == TokenType::UserString
        {
            result = true;
        }
        else {}
    }
    else {}
    result
}
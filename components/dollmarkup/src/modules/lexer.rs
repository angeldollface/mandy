/*
DOLLMARKUP by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing Mandy's error
/// struct.
use merrors::MandyError;


/// Defining an enum
/// to group the different
/// variants of possible
/// tokens.
#[derive(Clone, Debug,PartialEq)]
pub enum TokenType {
    UserString,
    Comment,
    Number,
    Assign,
    BooleanTrue,
    BooleanFalse,
    Section
}

/// A structure to capture
/// a lexed token.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub column: usize
}

/// Implementing generic
/// methods for the "Token"
/// structure.
impl Token {

    /// A standard method
    /// to create a new "instance"
    /// of this structure.
    pub fn new(
        value: &str,
        token_type: &TokenType,
        column: &usize
    ) -> Token {
        Token {
            value: value.to_string(),
            token_type: token_type.to_owned(),
            column: column.to_owned()
        }
    }

    /// For debugging and display we make
    /// a "to_string()" method.
    pub fn to_string(&self) -> String {
        let mut token_type_string: String = "".to_string();
        match &self.token_type{
            TokenType::UserString => token_type_string = "user_string".to_string(),
            TokenType::Comment => token_type_string = "comment".to_string(),
            TokenType::Number => token_type_string = "number".to_string(),
            TokenType::BooleanTrue => token_type_string = "true".to_string(),
            TokenType::BooleanFalse => token_type_string = "false".to_string(),
            TokenType::Assign => token_type_string = "assign".to_string(),
            TokenType::Section => token_type_string = "section".to_string()
        }
        format!("Value: {}\nType: {}\nColumn: {}", &self.value, token_type_string, &self.column)
    }

}

/// A method to attempt
/// to tokenize a passed-in
/// string. Returns an error
/// if this fails.
pub fn tokenize(
    subject: &String
) -> Result<Vec<Token>, MandyError>{
    let mut result: Vec<Token> = Vec::new();
    let chars: Vec<char> = subject.chars().collect();
    let input_length: usize = chars.len();
    let mut counter: usize = 0;
    let mut char_pool: Vec<char> = Vec::new();
    while counter < input_length{
        char_pool.push(chars[counter]);
        let joined: String = char_pool.clone().into_iter().collect();
        if is_bool_false(&joined){
            result.push(Token::new(&joined, &TokenType::BooleanFalse, &counter));
            char_pool = Vec::new();
        }
        if is_bool_true(&joined){
            result.push(Token::new(&joined, &TokenType::BooleanTrue, &counter));
            char_pool = Vec::new();
        }
        if is_string(&joined){
            result.push(Token::new(&joined, &TokenType::UserString, &counter));
            char_pool = Vec::new();
        }
        if is_comment(&joined){
            result.push(Token::new(&joined, &TokenType::Comment, &counter));
            char_pool = Vec::new();
        }
        if is_assign(&joined){
            result.push(Token::new(&joined, &TokenType::Assign, &counter));
            char_pool = Vec::new();
        }
        if is_number(&joined){
            result.push(Token::new(&joined, &TokenType::Number, &counter));
            char_pool = Vec::new();
        }
        if is_section_id(&joined){
            result.push(Token::new(&joined, &TokenType::Section, &counter));
            char_pool = Vec::new();
        }
        else {}
        counter = counter + 1;
    }
    if result.is_empty(){
        let e: String = format!("No valid tokens could be lexed from column 0 to {}", input_length);
        return Err::<Vec<Token>, MandyError>(
            MandyError::new(&e.to_string())
        );
    }
    else {}
    Ok(result)
}

/// Checks whether the string submitted constitutes a
/// user-supplied string.
pub fn is_string(subject: &String) -> bool {
    let subject_chars: Vec<char> = subject.chars().collect();
    let mut result: bool = false;
    if subject_chars.len() > 4 {
        if subject_chars[0] == '"' &&
        subject_chars[subject_chars.len() - 1] == '"'
        {
            result = true;
        }
        else {}
    }
    else {}
    result
}

/// Checks whether the string submitted constitutes a
/// comment.
pub fn is_comment(subject: &String) -> bool {
    let subject_chars: Vec<char> = subject.chars().collect();
    let mut result: bool = false;
    if subject_chars.len() > 3 {
        if subject_chars[0] == '/' &&
            subject_chars[1] == '/' &&
            subject_chars[subject_chars.len()-1] == '\n'
        {
            result = true;
        }
        else {}
    }
    else {}
    result
}

/// Checks whether the string submitted constitutes a
/// boolean "true".
pub fn is_bool_true(subject: &String) -> bool {
    let mut result: bool = false;
    if subject == "true"{
        result = true;
    }
    else {}
    result
}

/// Checks whether the string submitted constitutes a
/// the assignment operator.
pub fn is_assign(subject: &String) -> bool {
    let mut result: bool = false;
    if subject == "=>"{
        result = true;
    }
    else {}
    result
}

/// Checks whether the string submitted constitutes a
/// boolean "false".
pub fn is_bool_false(subject: &String) -> bool {
    let mut result: bool = false;
    if subject == "false"{
        result = true;
    }
    else {}
    result
}

/// Checks whether the string submitted, constitutes a
/// number.
pub fn is_number(subject: &String) -> bool {
    let subject_chars: Vec<char> = subject.chars().collect();
    let mut result: bool = false;
    if subject_chars.len() > 2 {
        if subject_chars[0] == '(' &&
            subject_chars[subject_chars.len()-1] == ')'
        {
            result = true;
        }
        else {}
    }
    else {}
    result
}

/// Checks, whether the string submitted, constitutes a
/// section ID.
pub fn is_section_id(subject: &String) -> bool {
    let mut result: bool = false;
    if subject == ":section "{
        result = true;
    }
    else {}
    result
}


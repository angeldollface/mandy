/*
VENTO OXIDE by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/


/// Defining an enum
/// to group the different
/// variants of possible
/// tokens.
#[derive(Clone, Debug)]
pub enum TokenType {
    UserString,
    Comment,
    Tag
}

/// A structure to capture
/// a lexed token.
#[derive(Clone, Debug)]
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
        value: &String,
        token_type: &TokenType,
        column: &usize
    ) -> Token {
        Token {
            value: value.to_owned(),
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
            TokenType::Tag => token_type_string = "tag".to_string()
        }
        format!("Value: {}\nType: {}\nColumn: {}", &self.value, token_type_string, &self.column)
    }

}

/// A method to attempt
/// to tokenize a passed in
/// string. Returns an error
/// if this fails.
pub fn raw_tokenize(
    subject: &String
) -> Vec<Token>{
    let mut result: Vec<Token> = Vec::new();
    let chars: Vec<char> = subject.chars().collect();
    let input_length: usize = chars.len();
    let mut counter: usize = 0;
    let mut char_pool: Vec<char> = Vec::new();
    while counter < input_length{
        char_pool.push(chars[counter]);
        let joined: String = char_pool.clone().into_iter().collect();
        if is_comment(&joined){
            result.push(Token::new(&joined, &TokenType::Comment, &counter));
            char_pool = Vec::new();
        }
        else if is_string(&joined){
            result.push(Token::new(&joined, &TokenType::UserString, &counter));
            char_pool = Vec::new();
        }
        else if is_tag(&joined){
            result.push(Token::new(&joined, &TokenType::Tag, &counter));
            char_pool = Vec::new();
        }
        else {}
        counter = counter + 1;
    }
    result
}

pub fn is_tag(subject: &String) -> bool {
    let subject_chars: Vec<char> = subject.chars().collect();
    let mut result: bool = false;
    let last: char = subject_chars[subject_chars.len() - 1];
    if subject_chars[0] == '<' && last == '>'{
        result = true;
    }
    else {}
    result
}

pub fn is_string(subject: &String) -> bool {
    let subject_chars: Vec<char> = subject.chars().collect();
    let mut result: bool = false;
    let last: char = subject_chars[subject_chars.len() - 1];
    let penultimate: char = subject_chars[subject_chars.len() - 2];
    if subject_chars[0] == '{' &&
        subject_chars[1] == '{' && 
        last == '}' &&
        penultimate == '}'
    {
        result = true;
    }
    else {}
    result
}

pub fn is_comment(subject: &String) -> bool {
    let subject_chars: Vec<char> = subject.chars().collect();
    let last: char = subject_chars[subject_chars.len() - 1];
    let mut result: bool = false;
    if subject_chars[0] == '#' &&
       last == '#'
    {
        result = true;
    }
    else {}
    result
}
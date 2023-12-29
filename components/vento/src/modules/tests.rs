use super::lexer::Token;
use super::lexer::TokenType;
use super::lexer::raw_tokenize;

#[test]
pub fn test_h1() {
    let code: String = "<h1>{{ message }}</h1>".to_string();
    let res: Vec<Token> = raw_tokenize(&code);
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

#[test]
pub fn test_comment() {
    let code: String = "{{# message #}}".to_string();
    let res: Vec<Token> = raw_tokenize(&code);
    let mut res_vec: Vec<Token> = Vec::new();
    res_vec.push(
        Token::new(&"{{# message #}}",&TokenType::Comment,&14)
    );
    assert_eq!(res, res_vec);
}
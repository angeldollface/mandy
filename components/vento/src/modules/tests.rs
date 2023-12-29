use super::lexer::Token;
use super::lexer::raw_tokenize;

#[test]
pub fn test_h1() {
    let code: String = "<h1>{{ `message {}}` }}</h1>".to_string();
    let res: Vec<Token> = raw_tokenize(&code);
    for token in res {
        println!("{}", token.to_string());
    }
}
/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

mod lib;
use lib::lexer::lex;
use lib::lexer::Token;
use lib::patterns::patterns;
use lib::lexer::TokenStream;

fn main() {
    let src: String = String::from("String name = 'Hello World!'\ntrip name;");
    let tokens: Vec<Token> = lex(&src, &patterns());
    let token_stream: TokenStream = TokenStream{
        tokens: tokens
    };
    println!("{}", token_stream.to_json());
}
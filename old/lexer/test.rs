#![cfg(test)]

use crate::token::Token;
use crate::lexer::Lexer;

#[test]
fn test_lexer() {
    let input = "
        let five = 5;
        let ten = 10;
        let add = fn(a, b) {
            a + b;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        5 == 5;
        5 != 10;";

    let tests = vec![
        Token::Let,
        Token::Ident("five"),
        Token::Assign,
        Token::Int("5"),
        Token::Semicolon,

        Token::Let,
        Token::Ident("ten"),
        Token::Assign,
        Token::Int("10"),
        Token::Semicolon,

        Token::Let,
        Token::Ident("add"),
        Token::Assign,
        Token::Function,
        Token::LParenthesis,
        Token::Ident("a"),
        Token::Comma,
        Token::Ident("b"),
        Token::RParenthesis,
        Token::LCurlyBracket,
        Token::Ident("a"),
        Token::Plus,
        Token::Ident("b"),
        Token::Semicolon,
        Token::RCurlyBracket,
        Token::Semicolon,

        Token::Let,
        Token::Ident("result"),
        Token::Assign,
        Token::Ident("add"),
        Token::LParenthesis,
        Token::Ident("five"),
        Token::Comma,
        Token::Ident("ten"),
        Token::RParenthesis,
        Token::Semicolon,

        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Int("5"),
        Token::Semicolon,

        Token::Int("5"),
        Token::LT,
        Token::Int("10"),
        Token::GT,
        Token::Int("5"),
        Token::Semicolon,

        Token::If,
        Token::LParenthesis,
        Token::Int("5"),
        Token::LT,
        Token::Int("10"),
        Token::RParenthesis,
        Token::LCurlyBracket,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RCurlyBracket,
        Token::Else,
        Token::LCurlyBracket,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RCurlyBracket,

        Token::Int("5"),
        Token::Eq,
        Token::Int("5"),
        Token::Semicolon,

        Token::Int("5"),
        Token::NotEq,
        Token::Int("10"),
        Token::Semicolon,
    ];

    let lexer  = Lexer::new(input);
    let tokens = lexer.tokenize();
    if tokens.len() != tests.len() {
        panic!("Number of toknes is different: expect {}, but found {}",
               tests.len(), tokens.len());
    } else {
        for (target, test) in tokens.iter().zip(&tests) {
            assert!(target == test,
                    "Assertion falied: expect {:?}, but found {:?}",
                    test, target);
        }
    }
}

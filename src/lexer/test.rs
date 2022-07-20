#![cfg(test)]

use crate::token::{Token, TokenType};
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
        Token::new(TokenType::Let,           "let"),
        Token::new(TokenType::Ident,         "five"),
        Token::new(TokenType::Assign,        "="),
        Token::new(TokenType::Int,           "5"),
        Token::new(TokenType::Semicolon,     ";"),

        Token::new(TokenType::Let,           "let"),
        Token::new(TokenType::Ident,         "ten"),
        Token::new(TokenType::Assign,        "="),
        Token::new(TokenType::Int,           "10"),
        Token::new(TokenType::Semicolon,     ";"),

        Token::new(TokenType::Let,           "let"),
        Token::new(TokenType::Ident,         "add"),
        Token::new(TokenType::Assign,        "="),
        Token::new(TokenType::Function,      "fn"),
        Token::new(TokenType::LParenthesis,  "("),
        Token::new(TokenType::Ident,         "a"),
        Token::new(TokenType::Comma,         ","),
        Token::new(TokenType::Ident,         "b"),
        Token::new(TokenType::RParenthesis,  ")"),
        Token::new(TokenType::LCurlyBracket, "{"),
        Token::new(TokenType::Ident,         "a"),
        Token::new(TokenType::Plus,          "+"),
        Token::new(TokenType::Ident,         "b"),
        Token::new(TokenType::Semicolon,     ";"),
        Token::new(TokenType::RCurlyBracket, "}"),
        Token::new(TokenType::Semicolon,     ";"),

        Token::new(TokenType::Let,           "let"),
        Token::new(TokenType::Ident,         "result"),
        Token::new(TokenType::Assign,        "="),
        Token::new(TokenType::Ident,         "add"),
        Token::new(TokenType::LParenthesis,  "("),
        Token::new(TokenType::Ident,         "five"),
        Token::new(TokenType::Comma,         ","),
        Token::new(TokenType::Ident,         "ten"),
        Token::new(TokenType::RParenthesis,  ")"),
        Token::new(TokenType::Semicolon,     ";"),

        Token::new(TokenType::Bang,          "!"),
        Token::new(TokenType::Minus,         "-"),
        Token::new(TokenType::Slash,         "/"),
        Token::new(TokenType::Asterisk,      "*"),
        Token::new(TokenType::Int,           "5"),
        Token::new(TokenType::Semicolon,     ";"),

        Token::new(TokenType::Int,           "5"),
        Token::new(TokenType::LT,            "<"),
        Token::new(TokenType::Int,           "10"),
        Token::new(TokenType::GT,            ">"),
        Token::new(TokenType::Int,           "5"),
        Token::new(TokenType::Semicolon,     ";"),

        Token::new(TokenType::If,            "if"),
        Token::new(TokenType::LParenthesis,  "("),
        Token::new(TokenType::Int,           "5"),
        Token::new(TokenType::LT,            "<"),
        Token::new(TokenType::Int,           "10"),
        Token::new(TokenType::RParenthesis,  ")"),
        Token::new(TokenType::LCurlyBracket, "{"),
        Token::new(TokenType::Return,        "return"),
        Token::new(TokenType::True,          "true"),
        Token::new(TokenType::Semicolon,     ";"),
        Token::new(TokenType::RCurlyBracket, "}"),
        Token::new(TokenType::Else,          "else"),
        Token::new(TokenType::LCurlyBracket, "{"),
        Token::new(TokenType::Return,        "return"),
        Token::new(TokenType::False,         "false"),
        Token::new(TokenType::Semicolon,     ";"),
        Token::new(TokenType::RCurlyBracket, "}"),

        Token::new(TokenType::Int,           "5"),
        Token::new(TokenType::Eq,            "=="),
        Token::new(TokenType::Int,           "5"),
        Token::new(TokenType::Semicolon,     ";"),

        Token::new(TokenType::Int,           "5"),
        Token::new(TokenType::NotEq,         "!="),
        Token::new(TokenType::Int,           "10"),
        Token::new(TokenType::Semicolon,     ";"),
    ];

    let lexer  = Lexer::new(input);
    let tokens = lexer.tokenize();
    if tokens.len() != tests.len() {
        panic!("Number of toknes is different: expect {}, but found {}",
               tests.len(), tokens.len());
    } else {
        for (target, test) in tokens.iter().zip(&tests) {
            assert!(target == test,
                    "Assertion failed: expect {:?}, got {:?}",
                    test, target);
        }
    }
}

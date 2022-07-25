#![cfg(test)]

use crate::token::{Token, TokenKind};
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
        5 != 10;
        \"hello world\";";

    let tests = vec![
        Token::new(TokenKind::Let,           ""),    
        Token::new(TokenKind::Ident,     "five"),
        Token::new(TokenKind::Assign,        ""),
        Token::new(TokenKind::Int,          "5"),
        Token::new(TokenKind::Semicolon,     ""),

        Token::new(TokenKind::Let,           ""),
        Token::new(TokenKind::Ident,      "ten"),
        Token::new(TokenKind::Assign,        ""),
        Token::new(TokenKind::Int,         "10"),
        Token::new(TokenKind::Semicolon,     ""),

        Token::new(TokenKind::Let,           ""),
        Token::new(TokenKind::Ident,      "add"),
        Token::new(TokenKind::Assign,        ""),
        Token::new(TokenKind::Function,      ""),
        Token::new(TokenKind::LParenthesis,  ""),
        Token::new(TokenKind::Ident,        "a"),
        Token::new(TokenKind::Comma,         ""),
        Token::new(TokenKind::Ident,        "b"),
        Token::new(TokenKind::RParenthesis,  ""),
        Token::new(TokenKind::LCurlyBracket, ""),
        Token::new(TokenKind::Ident,        "a"),
        Token::new(TokenKind::Plus,          ""),
        Token::new(TokenKind::Ident,        "b"),
        Token::new(TokenKind::Semicolon,     ""),
        Token::new(TokenKind::RCurlyBracket, ""),
        Token::new(TokenKind::Semicolon,     ""),
                                            
        Token::new(TokenKind::Let,           ""),
        Token::new(TokenKind::Ident,   "result"),
        Token::new(TokenKind::Assign,        ""),
        Token::new(TokenKind::Ident,      "add"),
        Token::new(TokenKind::LParenthesis,  ""),
        Token::new(TokenKind::Ident,     "five"),
        Token::new(TokenKind::Comma,         ""),
        Token::new(TokenKind::Ident,      "ten"),
        Token::new(TokenKind::RParenthesis,  ""),
        Token::new(TokenKind::Semicolon,     ""),
                                            
        Token::new(TokenKind::Bang,          ""),
        Token::new(TokenKind::Minus,         ""),
        Token::new(TokenKind::Slash,         ""),
        Token::new(TokenKind::Asterisk,      ""),
        Token::new(TokenKind::Int,          "5"),
        Token::new(TokenKind::Semicolon,     ""),
                                            
        Token::new(TokenKind::Int,          "5"),
        Token::new(TokenKind::LT,            ""),
        Token::new(TokenKind::Int,         "10"),
        Token::new(TokenKind::GT,            ""),
        Token::new(TokenKind::Int,          "5"),
        Token::new(TokenKind::Semicolon,     ""),
                                            
        Token::new(TokenKind::If,            ""),
        Token::new(TokenKind::LParenthesis,  ""),
        Token::new(TokenKind::Int,          "5"),
        Token::new(TokenKind::LT,            ""),
        Token::new(TokenKind::Int,         "10"),
        Token::new(TokenKind::RParenthesis,  ""),
        Token::new(TokenKind::LCurlyBracket, ""),
        Token::new(TokenKind::Return,        ""),
        Token::new(TokenKind::True,          ""),
        Token::new(TokenKind::Semicolon,     ""),
        Token::new(TokenKind::RCurlyBracket, ""),
        Token::new(TokenKind::Else,          ""),
        Token::new(TokenKind::LCurlyBracket, ""),
        Token::new(TokenKind::Return,        ""),
        Token::new(TokenKind::False,         ""),
        Token::new(TokenKind::Semicolon,     ""),
        Token::new(TokenKind::RCurlyBracket, ""),
                                            
        Token::new(TokenKind::Int,          "5"),
        Token::new(TokenKind::Eq,            ""),
        Token::new(TokenKind::Int,          "5"),
        Token::new(TokenKind::Semicolon,     ""),

        Token::new(TokenKind::Int,          "5"),
        Token::new(TokenKind::NotEq,         ""),
        Token::new(TokenKind::Int,         "10"),
        Token::new(TokenKind::Semicolon,     ""),

        Token::new(TokenKind::Str, "hello world"),
        Token::new(TokenKind::Semicolon,     ""),

        Token::new(TokenKind::Eof,            ""),
    ];

    let lexer  = Lexer::new(input);
    let tokens = lexer.tokenize();
    if tokens.len() != tests.len() {
        println!("{:?}", tokens);
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

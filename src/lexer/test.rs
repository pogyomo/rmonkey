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
        5 != 10;";

    let tests = vec![
        Token::new(TokenKind::Let,           None),    
        Token::new(TokenKind::Ident,         Some("five")),
        Token::new(TokenKind::Assign,        None),
        Token::new(TokenKind::Int,           Some("5")),
        Token::new(TokenKind::Semicolon,     None),

        Token::new(TokenKind::Let,           None),
        Token::new(TokenKind::Ident,         Some("ten")),
        Token::new(TokenKind::Assign,        None),
        Token::new(TokenKind::Int,           Some("10")),
        Token::new(TokenKind::Semicolon,     None),

        Token::new(TokenKind::Let,           None),
        Token::new(TokenKind::Ident,         Some("add")),
        Token::new(TokenKind::Assign,        None),
        Token::new(TokenKind::Function,      None),
        Token::new(TokenKind::LParenthesis,  None),
        Token::new(TokenKind::Ident,         Some("a")),
        Token::new(TokenKind::Comma,         None),
        Token::new(TokenKind::Ident,         Some("b")),
        Token::new(TokenKind::RParenthesis,  None),
        Token::new(TokenKind::LCurlyBracket, None),
        Token::new(TokenKind::Ident,         Some("a")),
        Token::new(TokenKind::Plus,          None),
        Token::new(TokenKind::Ident,         Some("b")),
        Token::new(TokenKind::Semicolon,     None),
        Token::new(TokenKind::RCurlyBracket, None),
        Token::new(TokenKind::Semicolon,     None),
                                            
        Token::new(TokenKind::Let,           None),
        Token::new(TokenKind::Ident,         Some("result")),
        Token::new(TokenKind::Assign,        None),
        Token::new(TokenKind::Ident,         Some("add")),
        Token::new(TokenKind::LParenthesis,  None),
        Token::new(TokenKind::Ident,         Some("five")),
        Token::new(TokenKind::Comma,         None),
        Token::new(TokenKind::Ident,         Some("ten")),
        Token::new(TokenKind::RParenthesis,  None),
        Token::new(TokenKind::Semicolon,     None),
                                            
        Token::new(TokenKind::Bang,          None),
        Token::new(TokenKind::Minus,         None),
        Token::new(TokenKind::Slash,         None),
        Token::new(TokenKind::Asterisk,      None),
        Token::new(TokenKind::Int,           Some("5")),
        Token::new(TokenKind::Semicolon,     None),
                                            
        Token::new(TokenKind::Int,           Some("5")),
        Token::new(TokenKind::LT,            None),
        Token::new(TokenKind::Int,           Some("10")),
        Token::new(TokenKind::GT,            None),
        Token::new(TokenKind::Int,           Some("5")),
        Token::new(TokenKind::Semicolon,     None),
                                            
        Token::new(TokenKind::If,            None),
        Token::new(TokenKind::LParenthesis,  None),
        Token::new(TokenKind::Int,           Some("5")),
        Token::new(TokenKind::LT,            None),
        Token::new(TokenKind::Int,           Some("10")),
        Token::new(TokenKind::RParenthesis,  None),
        Token::new(TokenKind::LCurlyBracket, None),
        Token::new(TokenKind::Return,        None),
        Token::new(TokenKind::True,          None),
        Token::new(TokenKind::Semicolon,     None),
        Token::new(TokenKind::RCurlyBracket, None),
        Token::new(TokenKind::Else,          None),
        Token::new(TokenKind::LCurlyBracket, None),
        Token::new(TokenKind::Return,        None),
        Token::new(TokenKind::False,         None),
        Token::new(TokenKind::Semicolon,     None),
        Token::new(TokenKind::RCurlyBracket, None),
                                            
        Token::new(TokenKind::Int,           Some("5")),
        Token::new(TokenKind::Eq,            None),
        Token::new(TokenKind::Int,           Some("5")),
        Token::new(TokenKind::Semicolon,     None),

        Token::new(TokenKind::Int,           Some("5")),
        Token::new(TokenKind::NotEq,         None),
        Token::new(TokenKind::Int,           Some("10")),
        Token::new(TokenKind::Semicolon,     None),
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

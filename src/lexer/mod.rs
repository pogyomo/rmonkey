mod test;

use std::{cell::Cell, collections::HashMap};
use crate::token::{Token, TokenType};
use once_cell::sync::Lazy;

pub struct Lexer<'a> {
    input: Cell<&'a str>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer { input: Cell::new(input) }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let mut ret = Vec::new();
        while let Some(token) = self.token() {
            ret.push(token);
        }
        ret
    }
}

impl<'a> Lexer<'a> {
    fn token(&self) -> Option<Token> {
        self.input.set(self.input.get().trim_start());

        if let Some(token) = self.ident_or_keyword() {
           return Some(token);
        }
        if let Some(token) = self.integer() {
            return Some(token);
        }
        if let Some(token) = self.one_or_more() {
            return Some(token);
        }
        None
    }

    fn ident_or_keyword(&self) -> Option<Token> {
        static KEYWORD: Lazy<HashMap<&str, Token>> = Lazy::new(|| {
            HashMap::from([
                ("fn",     Token::new(TokenType::Function, "fn")),
                ("let",    Token::new(TokenType::Let,      "let")),
                ("true",   Token::new(TokenType::True,     "true")),
                ("false",  Token::new(TokenType::False,    "false")),
                ("if",     Token::new(TokenType::If,       "if")),
                ("else",   Token::new(TokenType::Else,     "else")),
                ("return", Token::new(TokenType::Return,   "return")),
            ])
        });

        if self.input.get().chars().next()?.is_ascii_alphabetic() {
            let body = self.trim_start_with(|c: char| {
                c.is_ascii_alphanumeric() || c == '_'
            });
            match KEYWORD.get(body.to_lowercase().as_str()) {
                Some(token) => Some(*token),
                None        => Some(Token::new(TokenType::Ident, body)),
            }
        } else {
            None
        }
    }

    fn integer(&self) -> Option<Token> {
        if self.input.get().chars().next()?.is_ascii_digit() {
            let body = self.trim_start_with(|c: char| c.is_ascii_digit());
            Some(Token::new(TokenType::Int, body))
        } else {
            None
        }
    }

    fn one_or_more(&self) -> Option<Token> {
        let mut chars = self.input.get().chars();
        let ret = match chars.next()? {
            '=' => {
                if self.input.get().chars().nth(1)? == '=' {
                    chars.next();
                    Some(Token::new(TokenType::Eq,     "=="))
                } else {
                    Some(Token::new(TokenType::Assign, "="))
                }
            }
            '!' => {
                if self.input.get().chars().nth(1)? == '=' {
                    chars.next();
                    Some(Token::new(TokenType::NotEq, "!="))
                } else {
                    Some(Token::new(TokenType::Bang,  "!"))
                }
            }
            '+' => Some(Token::new(TokenType::Plus,          "+")),
            '-' => Some(Token::new(TokenType::Minus,         "-")),
            '*' => Some(Token::new(TokenType::Asterisk,      "*")),
            '/' => Some(Token::new(TokenType::Slash,         "/")),
            '<' => Some(Token::new(TokenType::LT,            "<")),
            '>' => Some(Token::new(TokenType::GT,            ">")),
            ',' => Some(Token::new(TokenType::Comma,         ",")),
            ';' => Some(Token::new(TokenType::Semicolon,     ";")),
            '(' => Some(Token::new(TokenType::LParenthesis,  "(")),
            ')' => Some(Token::new(TokenType::RParenthesis,  ")")),
            '{' => Some(Token::new(TokenType::LCurlyBracket, "{")),
            '}' => Some(Token::new(TokenType::RCurlyBracket, "}")),
            _   => return None,
        };
        self.input.set(chars.as_str());
        ret
    }

    // Take a closure and get trimmed string from begin of self.input
    fn trim_start_with<F>(&self, f: F) -> &str
    where
        F: Fn(char) -> bool,
    {
        let input = self.input.get();
        let sep = input.find(|c: char| !f(c)).unwrap_or(input.len());
        let (ret, other) = input.split_at(sep);
        self.input.set(other);
        ret
    }
}

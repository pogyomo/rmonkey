mod test;

use std::{cell::Cell, collections::HashMap};
use crate::token::{Token, TokenKind};
use once_cell::sync::Lazy;

pub struct Lexer<'a> {
    input: Cell<&'a str>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer<'_> {
        Lexer { input: Cell::new(input) }
    }

    pub fn tokenize(&self) -> Vec<Token<'_>> {
        let mut ret = Vec::new();
        while let Some(token) = self.token() {
            ret.push(token);
        }
        ret.push(Token::new(TokenKind::Eof, ""));
        ret
    }
}

impl <'a> Lexer<'a> {
    fn token(&self) -> Option<Token<'_>> {
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

    fn ident_or_keyword(&self) -> Option<Token<'_>> {
        static KEYWORD: Lazy<HashMap<&str, Token>> = Lazy::new(|| {
            HashMap::from([
                ("fn",     Token::new(TokenKind::Function, "")),
                ("let",    Token::new(TokenKind::Let,      "")),
                ("true",   Token::new(TokenKind::True,     "")),
                ("false",  Token::new(TokenKind::False,    "")),
                ("if",     Token::new(TokenKind::If,       "")),
                ("else",   Token::new(TokenKind::Else,     "")),
                ("return", Token::new(TokenKind::Return,   "")),
            ])
        });

        if self.input.get().chars().next()?.is_ascii_alphabetic() {
            let body = self.trim_start_with(|c: char| {
                c.is_ascii_alphanumeric() || c == '_'
            });
            match KEYWORD.get(body.to_lowercase().as_str()) {
                Some(token) => Some(*token),
                None        => Some(Token::new(TokenKind::Ident, body)),
            }
        } else {
            None
        }
    }

    fn integer(&self) -> Option<Token> {
        if self.input.get().chars().next()?.is_ascii_digit() {
            let body = self.trim_start_with(|c: char| c.is_ascii_digit());
            Some(Token::new(TokenKind::Int, body))
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
                    Some(Token::new(TokenKind::Eq,     ""))
                } else {
                    Some(Token::new(TokenKind::Assign, ""))
                }
            }
            '!' => {
                if self.input.get().chars().nth(1)? == '=' {
                    chars.next();
                    Some(Token::new(TokenKind::NotEq, ""))
                } else {
                    Some(Token::new(TokenKind::Bang,  ""))
                }
            }
            '+' => {
                if self.input.get().chars().nth(1)? == '+' {
                    chars.next();
                    Some(Token::new(TokenKind::Inc,   ""))
                } else {
                    Some(Token::new(TokenKind::Plus,  ""))
                }
            }
            '-' => {
                if self.input.get().chars().nth(1)? == '-' {
                    chars.next();
                    Some(Token::new(TokenKind::Dec,    ""))
                } else {
                    Some(Token::new(TokenKind::Minus,  ""))
                }
            }
            '*' => Some(Token::new(TokenKind::Asterisk,      "")),
            '/' => Some(Token::new(TokenKind::Slash,         "")),
            '<' => Some(Token::new(TokenKind::LT,            "")),
            '>' => Some(Token::new(TokenKind::GT,            "")),
            ',' => Some(Token::new(TokenKind::Comma,         "")),
            ';' => Some(Token::new(TokenKind::Semicolon,     "")),
            '(' => Some(Token::new(TokenKind::LParenthesis,  "")),
            ')' => Some(Token::new(TokenKind::RParenthesis,  "")),
            '{' => Some(Token::new(TokenKind::LCurlyBracket, "")),
            '}' => Some(Token::new(TokenKind::RCurlyBracket, "")),
            '"' => return self.read_string(),
            _   => return None,
        };
        self.input.set(chars.as_str());
        ret
    }

    fn read_string(&self) -> Option<Token> {
        // Skip '"'
        self.skip_char();

        // Read string
        let ret = self.trim_start_with(|c: char| c != '"');

        // Skip '"'
        self.skip_char();

        Some(Token::new(TokenKind::Str, ret))
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

    fn skip_char(&self) {
        let mut chars = self.input.get().chars();
        chars.next();
        self.input.set(chars.as_str());
    }
}

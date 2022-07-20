#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub literal: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, literal: &str) -> Token {
        Token { token_type, literal }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    // Token with literal
    Ident,
    Int,

    // One-character token
    Assign,        // '='
    Plus,          // '+'
    Minus,         // '-'
    Bang,          // '!'
    Asterisk,      // '*'
    Slash,         // '/'
    LT,            // '<'
    GT,            // '>'
    Comma,         // ','
    Semicolon,     // ';'
    LParenthesis,  // '('
    RParenthesis,  // ')'
    LCurlyBracket, // '{'
    RCurlyBracket, // '}'

    // Two or more character token
    Eq,            // '=='
    NotEq,         // '!='

    // Keyword
    Function, // 'fn'
    Let,      // 'let'
    True,     // 'true'
    False,    // 'false'
    If,       // 'if'
    Else,     // 'else'
    Return,   // 'return'
}

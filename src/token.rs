#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub literal: Option<&'a str>,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, literal: Option<&str>) -> Token<'_> {
        Token { kind, literal }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
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

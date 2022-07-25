#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub literal: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, literal: &str) -> Token<'_> {
        Token { kind, literal }
    }

    pub fn literal(&self) -> &str {
        match self.kind {
            TokenKind::Eof => "eof",

            TokenKind::Ident
            | TokenKind::Int
            | TokenKind::Str => self.literal,

            TokenKind::Assign        => "=",
            TokenKind::Plus          => "+",
            TokenKind::Minus         => "-",
            TokenKind::Bang          => "!",
            TokenKind::Asterisk      => "*",
            TokenKind::Slash         => "/",
            TokenKind::LT            => "<",
            TokenKind::GT            => ">",
            TokenKind::Comma         => ",",
            TokenKind::Semicolon     => ";",
            TokenKind::LParenthesis  => "(",
            TokenKind::RParenthesis  => ")",
            TokenKind::LCurlyBracket => "{",
            TokenKind::RCurlyBracket => "}",

            TokenKind::Eq    => "==",
            TokenKind::NotEq => "!=",

            TokenKind::Function => "fn",
            TokenKind::Let      => "let",
            TokenKind::True     => "true",
            TokenKind::False    => "false",
            TokenKind::If       => "if",
            TokenKind::Else     => "else",
            TokenKind::Return   => "return",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    // Special token
    Eof,

    // Token with literal
    Ident,
    Int,
    Str,

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

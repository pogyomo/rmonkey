#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Token<'a> {
    Ident(&'a str),
    Int(&'a str),

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

impl<'a> Token<'a> {
    pub fn name(&self) -> &str {
        match self {
            Token::Ident(name)   => name,
            Token::Int(value)    => value,
            Token::Assign        => "=",
            Token::Plus          => "+",
            Token::Minus         => "-",
            Token::Bang          => "!",
            Token::Asterisk      => "*",
            Token::Slash         => "/",
            Token::LT            => "<",
            Token::GT            => ">",
            Token::Comma         => ",",
            Token::Semicolon     => ";",
            Token::LParenthesis  => "(",
            Token::RParenthesis  => ")",
            Token::LCurlyBracket => "{",
            Token::RCurlyBracket => "}",
            Token::Eq            => "==",
            Token::NotEq         => "!=",
            Token::Function      => "fn",
            Token::Let           => "let",
            Token::True          => "true",
            Token::False         => "false",
            Token::If            => "if",
            Token::Else          => "else",
            Token::Return        => "return",
        }
    }
}

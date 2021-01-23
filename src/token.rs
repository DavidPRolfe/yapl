#[derive(Debug)]
pub enum TokenMatch {
    // Special Tokens
    Illegal(String),

    // Literals + Identifier
    Identifier(String),
    Int(String),
    Float(String),
    String(String),
    True,
    False,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,

    // Operators and Comparisons
    Minus,
    Plus,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Keywords
    Fun,
    Loop,
    If,
    Else,
    Print,
    Return,
    Val,
    Var,
    Break,
    Continue
}

#[derive(Debug)]
pub struct Token {
    pub token_match: TokenMatch,
    pub char: usize,
    pub line: usize,
}

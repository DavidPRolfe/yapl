use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum TokenType {
    // Special Tokens
    Illegal(String),
    Semicolon,

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
    Continue,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub char: usize,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// This exists to make displaying tokens easier
#[derive(Debug)]
pub struct Tokens {
    tokens: Vec<Token>
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        for token in &self.tokens {
            write!(f, "{},", token)?;
        }
        write!(f, " ]")
    }
}
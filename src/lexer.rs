use std::iter::Peekable;

use crate::token::{ Token, TokenMatch };

pub struct Lexer<T: Iterator<Item = char>> {
    input_iter: Peekable<T>,
    char: usize,
    line: usize,
    newline: bool,
    last_match: TokenMatch,
    held_token: Option<Token>
}

impl<T: Iterator<Item = char>> Lexer<T> {
    pub fn new(iter: T) -> Self {
        Self {
            input_iter: iter.peekable(),
            char: 0,
            line: 1,
            newline: true,
            last_match: TokenMatch::Semicolon,
            held_token: None,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        if let Some(token) = self.held_token.take() {
            return Some(token);
        }

        while let Some(c) = self.advance() {
            let token_match = match c {
                c if c.is_whitespace() => continue,
                ';' => TokenMatch::Semicolon,
                '(' => TokenMatch::LeftParen,
                ')' => TokenMatch::RightParen,
                '{' => TokenMatch::LeftBrace,
                '}' => TokenMatch::RightBrace,
                ',' => TokenMatch::Comma,
                '-' => TokenMatch::Minus,
                '+' => TokenMatch::Plus,
                '*' => TokenMatch::Star,
                '<' => {
                    match self.peek() {
                        Some('=') => TokenMatch::LessEqual,
                        None | Some(_) => TokenMatch::Less
                    }
                }
                '>' => {
                    match self.peek() {
                        Some('=') => TokenMatch::GreaterEqual,
                        None | Some(_) => TokenMatch::Greater
                    }
                }
                '=' => {
                    match self.peek() {
                        Some('=') => TokenMatch::EqualEqual,
                        None | Some(_) => TokenMatch::Equal
                    }
                }
                '!' => {
                    match self.peek() {
                        Some('=') => TokenMatch::BangEqual,
                        None | Some(_) => TokenMatch::Bang
                    }
                }
                '/' => {
                    match self.peek() {
                        Some('/') => {
                            self.line_comment();
                            continue
                        }
                        None | Some(_) => TokenMatch::Slash
                    }
                }
                '"' => self.handle_string(),
                c if c.is_numeric() => self.handle_digits(c),
                c if c.is_alphabetic() || c == '_' => self.handle_letters(c),
                _ => TokenMatch::Illegal(c.to_string()),
            };
            self.last_match = token_match.clone();

            if let Some(token) = self.held_token.take() {
                self.held_token = Some(Token {
                    token_match,
                    char: self.char,
                    line: self.line,
                });
                return Some(token);
            }

            return Some(Token {
                token_match,
                char: self.char,
                line: self.line,
            });
        }
        if let Some(token) = self.held_token.take() {
            return Some(token);
        }

        None
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.input_iter.next();

        match next {
            None => {}
            Some('\n') => {
                match self.last_match {
                    TokenMatch::Identifier(_) | TokenMatch::Int(_) |
                    TokenMatch::Float(_) | TokenMatch::String(_) |
                    TokenMatch::True | TokenMatch::False | TokenMatch::RightParen |
                    TokenMatch::RightBrace | TokenMatch::Return | TokenMatch::Continue |
                    TokenMatch::Break => {
                        self.held_token = Some(Token {
                            token_match: TokenMatch::Semicolon,
                            char: self.char,
                            line: self.line,
                        })
                    }
                    _ => {}
                }
                self.char = 0;
                self.line += 1;
                self.newline = true;
            }
            Some(_) => {
                if self.newline {
                    self.newline = false
                } else {
                    self.char += 1
                }
            }
        }
        next
    }

    fn peek(&mut self) -> Option<&char> {
        self.input_iter.peek()
    }

    fn line_comment(&mut self) {
        while let Some(c) = self.peek() {
            if *c == '\n' {
                return
            }
            self.advance();
        }
    }

    fn handle_letters(&mut self, c: char) -> TokenMatch {
        let mut literal = String::from(c);
        loop {
            let c = self.peek();
            match c {
                None => break,
                Some(c) if c.is_alphanumeric() || *c == '_' => (),
                _ => break
            }
            let d = self.advance().unwrap();
            literal.push(d);
        }
        self.match_keywords(literal)
    }

    fn match_keywords(&self, s: String) -> TokenMatch {
        match s.as_str() {
            "true" => TokenMatch::True,
            "false" => TokenMatch::False,
            "fun" => TokenMatch::Fun,
            "loop" => TokenMatch::Loop,
            "if" => TokenMatch::If,
            "else" => TokenMatch::Else,
            "print" => TokenMatch::Print,
            "return" => TokenMatch::Return,
            "val" => TokenMatch::Val,
            "var" => TokenMatch::Var,
            "break" => TokenMatch::Break,
            "continue" => TokenMatch::Continue,

            _ => TokenMatch::Identifier(s)
        }
    }

    fn handle_digits(&mut self, c: char) -> TokenMatch {
        let mut literal = String::from(c);
        let mut is_float = false;
        loop {
            let c = self.peek();
            match c {
                None => break,
                Some('f') => {
                    self.advance();
                    is_float = true;
                    break
                }
                Some('.') => {
                    if is_float {
                        // This unwrap is safe because of the peek
                        let c = self.advance().unwrap();
                        literal.push(c);
                        return TokenMatch::Illegal(literal)
                    } else {
                        is_float = true;
                    }
                }
                Some(c) if c.is_numeric() => {}
                Some(_) => break
            }
            let d = self.advance().unwrap();
            literal.push(d)
        }
        if is_float {
            TokenMatch::Float(literal)
        } else {
            TokenMatch::Int(literal)
        }
    }

    fn handle_string(&mut self) -> TokenMatch {
        let mut literal = String::new();
        while let Some(c) = self.peek() {
            // this is a safe unwrap because we just peeked and its not a None
            let ch = self.advance().unwrap();
            match ch {
                '\n' => return TokenMatch::Illegal(literal),
                '"' => return TokenMatch::String(literal),
                _ => {}
            };
            // this is a safe unwrap because we just peeked and its not a None
            literal.push(ch)
        }
        TokenMatch::Illegal(literal)
    }
}

impl<T: Iterator<Item = char>> Iterator for Lexer<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

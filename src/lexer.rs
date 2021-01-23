use std::iter::Peekable;

#[derive(Debug)]
pub enum TokenMatch {
    // Special Tokens
    Illegal(String),
}

#[derive(Debug)]
pub struct Token {
    token_match: TokenMatch,
    char: usize,
    line: usize
}

pub struct Lexer<T: Iterator<Item = char>> {
    input_iter: Peekable<T>,
    char: usize,
    line: usize,
    newline: bool
}

impl<T: Iterator<Item = char>> Lexer<T> {
    pub fn new(iter: T) -> Self {
        Self { input_iter: iter.peekable(), char: 0, line: 1, newline: true }
    }

    fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.advance() {
            let token_type = match c {
                c if c.is_whitespace() => continue,
                '"' => self.handle_string(),
                c if c.is_numeric() => self.handle_digits(c),
                c if c.is_alphabetic() => self.handle_letters(c),
                _ => {
                    TokenMatch::Illegal(c.to_string())
                }
            };

            return Some(Token {
                token_match: token_type,
                char: self.char,
                line: self.line
            })
        }
        None
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.input_iter.next();

        match next {
            None => {},
            Some('\n') => {
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

    fn handle_letters(&mut self, c: char) -> TokenMatch {
        unimplemented!()
    }

    fn handle_digits(&mut self, c: char) -> TokenMatch {
        unimplemented!()
    }

    fn handle_string(&mut self) -> TokenMatch {
        unimplemented!()
    }
}

impl<T: Iterator<Item = char>> Iterator for Lexer<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
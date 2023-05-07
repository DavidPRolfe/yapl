mod ast;

use thiserror::Error;

pub use ast::*;

use crate::token::TokenType::*;
pub use crate::token::{Token, TokenType};

/*
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary | primary ;
primary        → INT | FLOAT | STRING | "true" | "false" | "(" expression ")" ;
 */

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("parse error - unexpected token `{0}`")]
    UnexpectedToken(Token),

    #[error("parse error - unexpected end of file")]
    EndOfFile
}

pub struct Parser<T: Iterator<Item = Token>> {
    input_iter: T,
    held: Option<Token>
}

impl<T: Iterator<Item = Token>> Parser<T> {
    pub fn new(iter: T) -> Self {
        Self {
            input_iter: iter,
            held: None
        }
    }

    // Parses the input and returns the resulting ast.
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expr()
    }

    // Grabs the held token if available or the next token from the input
    fn next(&mut self) -> Option<Token> {
        if let Some(token) = self.held.take() {
            return Some(token);
        }

        self.input_iter.next()
    }

    // Stores a token for a following call to next
    fn store(&mut self, token: Token) {
        self.held = Some(token);
    }

    fn expr(&mut self) -> Result<Expr, ParseError> {
        Ok(Expr::Equality(self.equality()?))
    }

    fn equality(&mut self) -> Result<Equality, ParseError> {
        let mut left = EqualityLeft::Comparison(self.comparison()?);
        let mut right: Option<EqualityRight>;

        loop {
            right = None;

            let token = match self.next() {
                None => break,
                Some(t) => t
            };

            let op = match token.token_type {
                EqualEqual => EqualityOp::Equal,
                BangEqual => EqualityOp::NotEqual,
                _ => {
                    self.store(token);
                    break
                }
            };

            right = Some(EqualityRight { op, right: self.comparison()? });
            left = EqualityLeft::Equality( Box::new(Equality {left, right }) );
        }

        Ok(Equality { left, right })
    }

    fn comparison(&mut self) -> Result<Comparison, ParseError> {
        let mut left = ComparisonLeft::Term(self.term()?);
        let mut right: Option<ComparisonRight>;

        loop {
            right = None;

            let token = match self.next() {
                None => break,
                Some(t) => t
            };

            let op = match token.token_type {
                Greater => ComparisonOp::Greater,
                GreaterEqual => ComparisonOp::GreaterEqual,
                Less => ComparisonOp::Less,
                LessEqual => ComparisonOp::LessEqual,
                _ => {
                    self.store(token);
                    break
                }
            };

            right = Some(ComparisonRight { op, right: self.term()? });
            left = ComparisonLeft::Comparison( Box::new(Comparison {left, right }) );
        }

        Ok(Comparison { left, right })
    }

    fn term(&mut self) -> Result<Term, ParseError> {
        let mut left = TermLeft::Factor(self.factor()?);
        let mut right: Option<TermRight>;

        loop {
            right = None;

            let token = match self.next() {
                None => break,
                Some(t) => t
            };

            let op = match token.token_type {
                Minus => TermOp::Minus,
                Plus => TermOp::Plus,
                _ => {
                    self.store(token);
                    break
                }
            };

            right = Some(TermRight { op, right: self.factor()? });
            left = TermLeft::Term( Box::new(Term {left, right }) );
        }

        Ok(Term { left, right })
    }

    fn factor(&mut self) -> Result<Factor, ParseError> {
        let mut left = FactorLeft::Unary(self.unary()?);
        let mut right: Option<FactorRight>;

        loop {
            right = None;

            let token = match self.next() {
                None => break,
                Some(t) => t
            };

            let op = match token.token_type {
                Slash => FactorOp::Div,
                Star => FactorOp::Mult,
                _ => {
                    self.store(token);
                    break
                }
            };

            right = Some(FactorRight { op, right: self.unary()? });
            left = FactorLeft::Factor( Box::new(Factor {left, right }) );
        }

        Ok(Factor { left, right })
    }

    fn unary(&mut self) -> Result<Unary, ParseError> {
        let token = self.next().ok_or(ParseError::EndOfFile)?;

        let mut op: Option<UnaryOp> = None;
        let mut matched = true;

        match token.token_type {
            Bang => op = Some(UnaryOp::Not),
            Minus => op = Some(UnaryOp::Minus),
            _ => matched = {
                self.store(token);
                false
            }
        }

        let right: Box<UnaryRight>;

        if matched {
            right = Box::new(UnaryRight::Unary(self.unary()?));
        } else {
            right = Box::new(UnaryRight::Primary(self.primary()?))
        }

        Ok(Unary{ op, right })
    }

    fn primary(&mut self) -> Result<Primary, ParseError> {
        let token = self.next().ok_or(ParseError::EndOfFile)?;

        match token.token_type {
            Int(lit) => Ok(Primary::Int(lit)),
            Float(lit) => Ok(Primary::Float(lit)),
            String(lit) => Ok(Primary::String(lit)),
            True => Ok(Primary::True),
            False => Ok(Primary::False),
            Identifier(lit) => Ok(Primary::Identifier(lit)),
            LeftParen => {
                let expr = self.expr()?;
                let right = self.next().ok_or(ParseError::EndOfFile)?;
                match right.token_type {
                    RightParen => Ok(Primary::Grouping(expr)),
                    _ => Err(ParseError::UnexpectedToken(right))
                }
            }
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }
}

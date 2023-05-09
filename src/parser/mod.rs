mod ast;

use thiserror::Error;

pub use ast::*;

use crate::token::TokenType::*;
use crate::token::TokenType::Identifier;
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
            held: None,
        }
    }

    /// Parses the input and returns the resulting ast.
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        self.program()
    }

    /// Grabs the held token if available or the next token from the input
    fn next(&mut self) -> Option<Token> {
        if let Some(token) = self.held.take() {
            return Some(token);
        }

        self.input_iter.next()
    }

    /// Stores a token for a following call to next
    fn store(&mut self, token: Token) {
        self.held = Some(token);
    }

    // Parsing rules
    fn program(&mut self) -> Result<Program, ParseError> {
        let mut program = Program { declarations: vec![] };

        loop {
            if let Some(token) = self.next() {
                self.store(token);
                program.declarations.push(self.declaration()?)
            } else { break }
        }

        Ok(program)
    }

    // Declarations

    fn declaration(&mut self) -> Result<Declaration, ParseError> {
        let token = self.next().ok_or(ParseError::EndOfFile)?;

        Ok(match token.token_type {
            Fun => {
                self.store(token);
                Declaration::Function(self.function()?)
            },
            Val | Var => {
                self.store(token);
                Declaration::Variable(self.variable()?)
            },
            _ => {
                self.store(token);
                Declaration::Statement(self.statement()?)
            }
        })
    }

    fn variable(&mut self) -> Result<Variable, ParseError> {
        let token = self.next().ok_or(ParseError::EndOfFile)?;
        let v_type = match token.token_type {
            Val => VariableType::Val,
            Var => VariableType::Var,
            _ => return Err(ParseError::UnexpectedToken(token))
        };

        let token = self.next().ok_or(ParseError::EndOfFile)?;
        let ident = match token.token_type {
            Identifier(s) => ast::Identifier(s),
            _ => return Err(ParseError::UnexpectedToken(token))
        };

        Ok(Variable {
            v_type,
            ident,
            value: self.expr()?,
        })
    }

    fn function(&mut self) -> Result<Function, ParseError> {
        let token = self.next().ok_or(ParseError::EndOfFile)?;
        if !matches!(token.token_type, Fun) {
            return Err(ParseError::UnexpectedToken(token))
        }

        let token = self.next().ok_or(ParseError::EndOfFile)?;
        let ident = match token.token_type {
            Identifier(s) => ast::Identifier(s),
            _ => return Err(ParseError::UnexpectedToken(token))
        };

        let token = self.next().ok_or(ParseError::EndOfFile)?;
        if !matches!(token.token_type, LeftParen) {
            return Err(ParseError::UnexpectedToken(token))
        }

        // TODO: Add arg handling

        let token = self.next().ok_or(ParseError::EndOfFile)?;
        if !matches!(token.token_type, RightParen) {
            return Err(ParseError::UnexpectedToken(token))
        }

        Ok(Function { ident, block: self.block()? })
    }

    fn statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.next().ok_or(ParseError::EndOfFile)?;

        Ok(match token.token_type { // TODO: add the rest
            Loop => {
                self.store(token);
                unimplemented!("Haven't added loops")
            },
            For => {
                self.store(token);
                unimplemented!("haven't added for loops")
            },
            Print => {
                self.store(token);
                unimplemented!("haven't added print")
            },
            Return => {
                self.store(token);
                unimplemented!("haven't added returns")
            },
            _ => {
                self.store(token);
                Statement::Expression(self.expr()?)
            }
        })
    }

    // Misc

    fn block(&mut self) -> Result<Block, ParseError> {
        let token = self.next().ok_or(ParseError::EndOfFile)?;
        if !matches!(token.token_type, LeftBrace) {
            return Err(ParseError::UnexpectedToken(token))
        }

        let mut block = Block { declarations: vec![] };

        loop {
            if let Some(token) = self.next() {
                if matches!(token.token_type, RightBrace) {
                    break
                }

                self.store(token);
                block.declarations.push(self.declaration()?)
            } else { break }
        }

        Ok(block)
    }

    // Expressions

    fn expr(&mut self) -> Result<Expr, ParseError> {
        Ok(Expr::LogicOr(self.logic_or()?))
    }

    fn logic_or(&mut self) -> Result<LogicOr, ParseError> {
        let mut left = LogicOrLeft::LogicAnd(self.logic_and()?);
        let mut right: Option<LogicAnd>;

        loop {
            right = None;

            if let Some(token) = self.next() {
                match token.token_type {
                    And => {},
                    _ => {
                        self.store(token);
                        break
                    }
                }
            } else {
                break
            }

            right = Some(self.logic_and()?);
            left = LogicOrLeft::LogicOr(Box::new(LogicOr { left, right }));
        }

        Ok(LogicOr { left, right })
    }

    fn logic_and(&mut self) -> Result<LogicAnd, ParseError> {
        let mut left = LogicAndLeft::Equality(self.equality()?);
        let mut right: Option<Equality>;

        loop {
            right = None;

            if let Some(token) = self.next() {
                match token.token_type {
                    And => {},
                    _ => {
                        self.store(token);
                        break
                    }
                }
            } else {
                break
            }

            right = Some(self.equality()?);
            left = LogicAndLeft::LogicAnd(Box::new(LogicAnd { left, right }));
        }

        Ok(LogicAnd { left, right })
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
            Identifier(lit) => Ok(Primary::Identifier(ast::Identifier(lit))),
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

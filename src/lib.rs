use std::fs::File;
use std::io::{Bytes, Read};
use utf8_decode::UnsafeDecoder;

use thiserror::Error;

mod lexer;
mod parser;
mod token;

use lexer::Lexer;
use parser::{ParseError, Parser};
use token::Tokens;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error(transparent)]
    ReadError(#[from] std::io::Error),

    #[error("encountered errors during lexing `{0}`")]
    LexError(Tokens),

    #[error("encountered an error during parsing `{0}`")]
    ParseError(#[from] ParseError),
}

// FileReader is used to read a stream of chars from a file
struct FileReader {
    iter: UnsafeDecoder<Bytes<File>>,
    err: Result<(), std::io::Error>,
}

impl FileReader {
    fn open(path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;

        Ok(Self {
            iter: UnsafeDecoder::new(file.bytes()),
            err: Ok(()),
        })
    }
}

impl Iterator for &mut FileReader {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(Err(e)) => {
                self.err = Err(e);
                None
            }
            Some(Ok(c)) => Some(c),
        }
    }
}

pub fn compile(path: &str) -> Result<(), CompilerError> {
    let mut reader = FileReader::open(&path)?;

    let lexer = Lexer::new(&mut reader);

    let ast = Parser::new(lexer).parse()?;

    print!("{:?}", ast);

    Ok(())
}

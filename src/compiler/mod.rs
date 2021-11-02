pub mod lexer;
pub mod parser;
pub mod token;

use lexer::Lexer;
use parser::{Parser, Tree};
use token::Token;

#[derive(Debug)]
pub enum Error {
    LexingError(String),
    ParsingError(String),
}

pub fn compile(source: Vec<char>) -> Result<Vec<Tree>, Error> {
    let mut lexer = Lexer::new(source);
    let tokens: Vec<Token>;
    match lexer.run() {
        Ok(t) => tokens = t,
        Err(e) => return Err(Error::LexingError(e)),
    }

    let mut parser = Parser::new(&tokens);

    match parser.run() {
        Ok(t) => Ok(t.to_vec()),
        Err(e) => Err(Error::ParsingError(e)),
    }
}

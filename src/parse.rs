use fallible_iterator::{FallibleIterator, Peekable};

use crate::{
  ast::{File, Stmt},
  token::Token,
};

pub fn parse<T, E>(tokens: T) -> Result<File, ParseError>
where
  T: FallibleIterator<Item = Token, Error = E>,
{
  let mut parser = Parser::new(tokens);
  parser.parse_file()
}

pub struct Parser<T, E>
where
  T: FallibleIterator<Item = Token, Error = E>,
{
  tokens: Peekable<T>,
}

impl<T, E> Parser<T, E>
where
  T: FallibleIterator<Item = Token, Error = E>,
{
  pub fn new(tokens: T) -> Parser<T, E> {
    Parser {
      tokens: tokens.peekable(),
    }
  }

  pub fn parse_file(&mut self) -> Result<File, ParseError> {
    Ok(File(vec![]))
  }
}

#[derive(Debug)]
pub enum ParseError {}

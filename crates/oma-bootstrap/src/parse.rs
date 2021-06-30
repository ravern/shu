use std::iter::Peekable;

use crate::lex::{LexError, Token};

pub enum ParseError {
  Lex(LexError),
}

pub struct Parser<T>
where
  T: Iterator<Item = Result<Token, LexError>>,
{
  tokens: Peekable<T>,
}

impl<T> Parser<T>
where
  T: Iterator<Item = Result<Token, LexError>>,
{
  pub fn new(tokens: T) -> Parser<T> {
    Parser {
      tokens: tokens.peekable(),
    }
  }
}

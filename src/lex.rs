use std::error::Error;

use fallible_iterator::{FallibleIterator, Peekable};
use thiserror::Error;

use crate::token::Token;

pub fn lex<S, E>(source: S) -> Result<Vec<Token>, LexError<E>>
where
  S: FallibleIterator<Item = char, Error = E>,
  E: Error + 'static,
{
  let mut lexer = Lexer::new(source);
  lexer.tokens().collect()
}

pub struct Lexer<S, E>
where
  S: FallibleIterator<Item = char, Error = E>,
  E: Error + 'static,
{
  source: Peekable<S>,
}

impl<S, E> Lexer<S, E>
where
  S: FallibleIterator<Item = char, Error = E>,
  E: Error + 'static,
{
  pub fn new(source: S) -> Lexer<S, E> {
    Lexer {
      source: source.peekable(),
    }
  }

  pub fn tokens(&mut self) -> Tokens<S, E> {
    Tokens(self)
  }

  pub fn lex_token(&mut self) -> Result<Option<Token>, LexError<E>> {
    match self.source.peek()? {
      Some(char) if char.is_digit(10) => self.lex_int_or_float().map(Some),
      _ => unimplemented!(),
    }
  }

  pub fn lex_int_or_float(&mut self) -> Result<Token, LexError<E>> {
    Ok(Token::Int(1))
  }
}

pub struct Tokens<'a, S, E>(&'a mut Lexer<S, E>)
where
  S: FallibleIterator<Item = char, Error = E>,
  E: Error + 'static;

impl<'a, S, E> FallibleIterator for Tokens<'a, S, E>
where
  S: FallibleIterator<Item = char, Error = E>,
  E: Error + 'static,
{
  type Item = Token;
  type Error = LexError<E>;

  fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
    self.0.lex_token()
  }
}

#[derive(Debug, Error)]
pub enum LexError<E>
where
  E: Error + 'static,
{
  #[error("{0}")]
  Read(#[from] E),
}

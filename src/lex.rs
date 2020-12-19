use fallible_iterator::{FallibleIterator, Peekable};

use crate::token::Token;

pub fn lex<S, E>(source: S) -> Result<Vec<Token>, LexError<E>>
where
  S: FallibleIterator<Item = char, Error = E>,
{
  let mut lexer = Lexer::new(source);
  lexer.tokens().collect()
}

pub struct Lexer<S, E>
where
  S: FallibleIterator<Item = char, Error = E>,
{
  source: Peekable<S>,
}

impl<S, E> Lexer<S, E>
where
  S: FallibleIterator<Item = char, Error = E>,
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
    Ok(None)
  }
}

pub struct Tokens<'a, S, E>(&'a mut Lexer<S, E>)
where
  S: FallibleIterator<Item = char, Error = E>;

impl<'a, S, E> FallibleIterator for Tokens<'a, S, E>
where
  S: FallibleIterator<Item = char, Error = E>,
{
  type Item = Token;
  type Error = LexError<E>;

  fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
    self.0.lex_token()
  }
}

#[derive(Debug)]
pub enum LexError<E> {
  Read(E),
}

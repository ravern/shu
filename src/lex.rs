use fallible_iterator::{FallibleIterator, Peekable};

use crate::token::Token;

pub fn lex<S, E>(source: S) -> Result<Vec<Token>, LexError<E>>
where
  S: FallibleIterator<Item = char, Error = E>,
{
  let mut lexer = Lexer::new(source);
  lexer.lex_all_tokens()
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

  pub fn lex_all_tokens(&mut self) -> Result<Vec<Token>, LexError<E>> {
    Ok(vec![])
  }

  pub fn lex_token(&mut self) -> Result<Option<Token>, LexError<E>> {
    Ok(None)
  }
}

#[derive(Debug)]
pub enum LexError<E> {
  Read(E),
}

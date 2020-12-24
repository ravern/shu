use std::iter::Peekable;

use crate::{
  common::span::{Pos, Span, Spanned},
  parse::token::Token,
};

pub struct Lexer<S>
where
  S: Iterator<Item = char>,
{
  source: Peekable<S>,
  line: usize,
  column: usize,
}

impl<S> Lexer<S>
where
  S: Iterator<Item = char>,
{
  pub fn new(source: S) -> Lexer<S> {
    Lexer {
      source: source.peekable(),
      line: 1,
      column: 1,
    }
  }

  pub fn next(&mut self) -> Option<Result<Spanned<Token>, LexError>> {
    if self.source.peek().is_none() {
      None
    } else {
      Some(self.token())
    }
  }

  pub fn token(&mut self) -> Result<Spanned<Token>, LexError> {
    let char = match self.source.peek() {
      Some(char) => char,
      None => {
        return Err(LexError::Unexpected {
          unexpected: Unexpected::Eof,
          expected: None,
        })
      }
    };

    let from = Pos::new(self.line, self.column);

    // Match against first char for all tokens, and perform early return
    // for non-single-char tokens.
    let token = match char {
      '.' => Token::Dot,
      '(' => Token::LParen,
      ')' => Token::RParen,
      '{' => Token::LBrace,
      '}' => Token::RBrace,
      '[' => Token::LBracket,
      ']' => Token::RBracket,
      _ => {
        return Err(LexError::Unexpected {
          unexpected: Unexpected::Char(*char),
          expected: None,
        })
      }
    };

    // Advance past matched char.
    self.advance().unwrap();

    Ok(Spanned::new(
      Span::new(from, Pos::new(self.line, self.column)),
      token,
    ))
  }

  pub fn advance(&mut self) -> Option<char> {
    let char = match self.source.next() {
      Some(char) => char,
      None => return None,
    };

    self.column += 1;

    if char == '\n' {
      self.line += 1;
    }

    Some(char)
  }
}

#[derive(Debug, PartialEq)]
pub enum LexError {
  Unexpected {
    unexpected: Unexpected,
    expected: Option<Vec<Expected>>,
  },
}

#[derive(Debug, PartialEq)]
pub enum Unexpected {
  Char(char),
  Eof,
}

#[derive(Debug, PartialEq)]
pub enum Expected {
  Char(char),
  Eof,
}

#[cfg(test)]
mod tests {
  use crate::{common::span::Spanned, parse::token::Token};

  use super::Lexer;

  #[test]
  fn parens() {
    let mut lexer = Lexer::new("(){}[]".chars());
    assert_eq!(
      lexer.next(),
      Some(Ok(Spanned::new(((1, 1), (1, 2)).into(), Token::LParen)))
    );
    assert_eq!(
      lexer.next(),
      Some(Ok(Spanned::new(((1, 2), (1, 3)).into(), Token::RParen,)))
    );
    assert_eq!(
      lexer.next(),
      Some(Ok(Spanned::new(((1, 3), (1, 4)).into(), Token::LBrace,)))
    );
    assert_eq!(
      lexer.next(),
      Some(Ok(Spanned::new(((1, 4), (1, 5)).into(), Token::RBrace,)))
    );
    assert_eq!(
      lexer.next(),
      Some(Ok(Spanned::new(((1, 5), (1, 6)).into(), Token::LBracket,)))
    );
    assert_eq!(
      lexer.next(),
      Some(Ok(Spanned::new(((1, 6), (1, 7)).into(), Token::RBracket,)))
    );
  }
}

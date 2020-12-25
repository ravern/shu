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
    let from = self.pos();

    // Match against first char for all tokens, and perform early return
    // for non-single-char tokens.
    let token = match self.peek() {
      '+' => Token::Add,
      '-' => Token::Sub,
      '*' => Token::Mul,
      '%' => Token::Rem,
      '.' => Token::Dot,
      '(' => Token::LParen,
      ')' => Token::RParen,
      '{' => Token::LBrace,
      '}' => Token::RBrace,
      '[' => Token::LBracket,
      ']' => Token::RBracket,
      '/' => return self.slash(),
      char => {
        return Err(LexError::Unexpected {
          pos: from,
          unexpected: Unexpected::Char(*char),
          expected: None,
        })
      }
    };

    self.advance();

    Ok(Spanned::new(Span::new(from, self.pos()), token))
  }

  fn slash(&mut self) -> Result<Spanned<Token>, LexError> {
    let from = self.pos();

    self.expect('/');

    let token = match self.peek() {
      '/' => return self.line_comment(),
      _ => Token::Div,
    };

    self.advance();

    Ok(Spanned::new(Span::new(from, self.pos()), token))
  }

  fn line_comment(&mut self) -> Result<Spanned<Token>, LexError> {
    // Exclude the preceding slashes from the comment span.
    self.expect('/');

    let from = self.pos();

    let mut chars = Vec::new();
    loop {
      match self.source.peek() {
        Some('\n') => break,
        Some(_) => chars.push(self.advance()),
        None => break,
      };
    }

    Ok(Spanned::new(
      Span::new(from, self.pos()),
      Token::Comment(chars.into_iter().collect()),
    ))
  }

  fn peek(&mut self) -> &char {
    self.source.peek().expect("no char to peek")
  }

  // Advances by one char and returns it.
  fn advance(&mut self) -> char {
    let char = self.source.next().expect("no char to advance");

    self.column += 1;
    if char == '\n' {
      self.line += 1;
    }

    char
  }

  // Advances and asserts that the char is the given one.
  fn expect(&mut self, expected_char: char) -> char {
    let char = self.advance();

    assert_eq!(
      char, expected_char,
      "expected char '{}', but got {}'",
      char, expected_char
    );

    char
  }

  fn pos(&self) -> Pos {
    Pos::new(self.line, self.column)
  }
}

#[derive(Debug, PartialEq)]
pub enum LexError {
  Unexpected {
    pos: Pos,
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
  fn line_comment() {
    let mut lexer = Lexer::new("// This is a test comment.".chars());
    assert_eq!(
      lexer.next(),
      Some(Ok(Spanned::new(
        ((1, 3), (1, 27)).into(),
        Token::Comment(" This is a test comment.".to_string())
      )))
    );
  }

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

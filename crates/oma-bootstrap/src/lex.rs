use crate::{
  span::{Source, Span, Spanned},
  token::Token,
};

#[derive(Debug, PartialEq)]
pub enum LexError {
  UnexpectedChar(u8),
  UnexpectedEof,
}

pub struct Lexer {
  source: Source,
  start: usize,
  end: usize,
}

impl Lexer {
  pub fn new(source: &str) -> Lexer {
    Lexer {
      source: Source::new(source),
      start: 0,
      end: 0,
    }
  }

  pub fn source(&self) -> &Source {
    &self.source
  }

  pub fn collect(mut self) -> Vec<Result<Spanned<Token>, Spanned<LexError>>> {
    let mut tokens = Vec::new();

    loop {
      let result = self.next();

      if let Ok(spanned_token) = &result {
        if let Token::Eof = spanned_token.base() {
          tokens.push(result);
          break;
        }
      }

      tokens.push(result);
    }

    tokens
  }

  pub fn next(&mut self) -> Result<Spanned<Token>, Spanned<LexError>> {
    self.whitespace();

    self.start = self.end;

    match self.peek() {
      Some(b'+') => self.build_and_advance(Token::Plus),
      Some(b'-') => self.build_and_advance(Token::Hyphen),
      Some(b'*') => self.build_and_advance(Token::Asterisk),
      Some(b'/') => {
        self.advance();
        match self.peek() {
          Some(b'/') => self.comment(),
          _ => self.build(Token::Slash),
        }
      }
      Some(b'\n') => self.build_and_advance(Token::Newline),
      Some(byte) if is_digit(byte) => self.number(),
      Some(byte) => self.build_err_and_advance(LexError::UnexpectedChar(byte)),
      None => self.build(Token::Eof),
    }
  }

  fn comment(&mut self) -> Result<Spanned<Token>, Spanned<LexError>> {
    loop {
      match self.peek() {
        Some(b'\n') => break,
        None => break,
        _ => {
          self.advance();
        }
      }
    }
    self.build(Token::Comment)
  }

  fn number(&mut self) -> Result<Spanned<Token>, Spanned<LexError>> {
    let mut is_float = false;

    loop {
      match self.peek() {
        Some(b'.') if !is_float => {
          is_float = true;
          self.advance();
        }
        Some(byte) if is_digit(byte) => {
          self.advance();
        }
        Some(_) => break,
        None => break,
      }
    }

    if is_float {
      self.build(Token::Float)
    } else {
      self.build(Token::Int)
    }
  }

  fn whitespace(&mut self) {
    loop {
      match self.peek() {
        Some(byte) if is_whitespace(byte) => {
          self.advance();
        }
        _ => break,
      }
    }
  }

  fn build(&self, token: Token) -> Result<Spanned<Token>, Spanned<LexError>> {
    Ok(Spanned::new(
      token,
      Span::new(self.source.clone(), self.start, self.end),
    ))
  }

  fn build_and_advance(
    &mut self,
    token: Token,
  ) -> Result<Spanned<Token>, Spanned<LexError>> {
    self.advance();
    self.build(token)
  }

  fn build_err(
    &self,
    error: LexError,
  ) -> Result<Spanned<Token>, Spanned<LexError>> {
    Err(Spanned::new(
      error,
      Span::new(self.source.clone(), self.start, self.end),
    ))
  }

  fn build_err_and_advance(
    &mut self,
    error: LexError,
  ) -> Result<Spanned<Token>, Spanned<LexError>> {
    self.advance();
    self.build_err(error)
  }

  fn peek(&self) -> Option<u8> {
    self.source.get(self.end)
  }

  fn advance(&mut self) -> Option<u8> {
    self.end += 1;
    self.source.get(self.end - 1)
  }

  fn expect(&mut self, expected: u8) -> Result<u8, LexError> {
    match self.advance() {
      Some(byte) if byte == expected => Ok(byte),
      Some(byte) => Err(LexError::UnexpectedChar(byte)),
      None => Err(LexError::UnexpectedEof),
    }
  }
}

fn is_digit(byte: u8) -> bool {
  byte >= b'0' && byte <= b'9'
}

fn is_whitespace(byte: u8) -> bool {
  byte == b'\r' || byte == b' ' || byte == b'\t'
}

#[cfg(test)]
mod tests {
  use crate::{
    span::{Span, Spanned},
    token::Token,
  };

  use super::Lexer;

  #[test]
  fn int() {
    let lexer = Lexer::new("42");
    let source = lexer.source().clone();
    let tokens = lexer.collect();

    assert_eq!(
      tokens,
      vec![
        Ok(Spanned::new(Token::Int, Span::new(source.clone(), 0, 2))),
        Ok(Spanned::new(Token::Eof, Span::new(source.clone(), 2, 2))),
      ]
    );
  }

  #[test]
  fn float() {
    let lexer = Lexer::new("3.14");
    let source = lexer.source().clone();
    let tokens = lexer.collect();

    assert_eq!(
      tokens,
      vec![
        Ok(Spanned::new(Token::Float, Span::new(source.clone(), 0, 4))),
        Ok(Spanned::new(Token::Eof, Span::new(source.clone(), 4, 4))),
      ]
    );
  }

  #[test]
  fn newline() {
    let lexer = Lexer::new("\n \n");
    let source = lexer.source().clone();
    let tokens = lexer.collect();

    assert_eq!(
      tokens,
      vec![
        Ok(Spanned::new(
          Token::Newline,
          Span::new(source.clone(), 0, 1)
        )),
        Ok(Spanned::new(
          Token::Newline,
          Span::new(source.clone(), 2, 3)
        )),
        Ok(Spanned::new(Token::Eof, Span::new(source.clone(), 3, 3))),
      ]
    );
  }

  #[test]
  fn comment() {
    let lexer = Lexer::new("// This is a comment");
    let source = lexer.source().clone();
    let tokens = lexer.collect();

    assert_eq!(
      tokens,
      vec![
        Ok(Spanned::new(
          Token::Comment,
          Span::new(source.clone(), 0, 20)
        )),
        Ok(Spanned::new(Token::Eof, Span::new(source.clone(), 20, 20))),
      ]
    );
  }

  #[test]
  fn punctuation() {
    let lexer = Lexer::new("+ - * /");
    let source = lexer.source().clone();
    let tokens = lexer.collect();

    assert_eq!(
      tokens,
      vec![
        Ok(Spanned::new(Token::Plus, Span::new(source.clone(), 0, 1))),
        Ok(Spanned::new(Token::Hyphen, Span::new(source.clone(), 2, 3))),
        Ok(Spanned::new(
          Token::Asterisk,
          Span::new(source.clone(), 4, 5)
        )),
        Ok(Spanned::new(Token::Slash, Span::new(source.clone(), 6, 7))),
        Ok(Spanned::new(Token::Eof, Span::new(source.clone(), 7, 7))),
      ]
    );
  }
}

use crate::{
  common::{Source, Span, Spanned},
  token::Token,
};

#[derive(Debug, PartialEq)]
pub enum LexError {
  UnexpectedChar(u8),
  UnexpectedEof,
}

pub struct Lexer {
  source: Source,
  line: usize,
  column: usize,
  start: usize,
  end: usize,
}

impl Lexer {
  pub fn new(source: &str) -> Lexer {
    Lexer {
      source: Source::new(source),
      line: 0,
      column: 0,
      start: 0,
      end: 0,
    }
  }

  pub fn source(&self) -> &Source {
    &self.source
  }

  pub fn collect(mut self) -> Vec<Result<Spanned<Token>, LexError>> {
    let mut tokens = Vec::new();

    loop {
      let token_result = self.next();

      if let Ok(token) = &token_result {
        if let Token::Eof = token.base() {
          tokens.push(token_result);
          break;
        }
      }

      tokens.push(token_result)
    }

    tokens
  }

  pub fn next(&mut self) -> Result<Spanned<Token>, LexError> {
    self.start = self.end;

    let token = match self.peek() {
      Some(b'+') => self.build_and_advance(Token::Plus),
      Some(b'-') => self.build_and_advance(Token::Hyphen),
      Some(b'*') => self.build_and_advance(Token::Asterisk),
      Some(b'/') => self.build_and_advance(Token::Slash),
      Some(byte) if is_digit(byte) => self.number()?,
      Some(byte) => return Err(LexError::UnexpectedChar(byte)),
      None => self.build(Token::Eof),
    };

    Ok(token)
  }

  fn number(&mut self) -> Result<Spanned<Token>, LexError> {
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
      Ok(self.build(Token::Float))
    } else {
      Ok(self.build(Token::Int))
    }
  }

  fn build(&self, token: Token) -> Spanned<Token> {
    Spanned::new(
      token,
      Span::new(
        self.source.clone(),
        self.line,
        self.column,
        self.start,
        self.end,
      ),
    )
  }

  fn build_and_advance(&mut self, token: Token) -> Spanned<Token> {
    self.advance();
    self.build(token)
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

#[cfg(test)]
mod tests {
  use crate::{
    common::{Span, Spanned},
    token::Token,
  };

  use super::Lexer;

  #[test]
  fn int() {
    let mut lexer = Lexer::new("1+1");
    let source = lexer.source().clone();
    let tokens = lexer.collect();

    assert_eq!(
      tokens,
      vec![Ok(Spanned::new(
        Token::Int,
        Span::new(source.clone(), 0, 0, 0, 0)
      ))]
    );
  }
}

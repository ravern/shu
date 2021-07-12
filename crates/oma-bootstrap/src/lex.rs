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
      Some(b'"') => self.string(),
      Some(b'+') => self.advance_and_build(Token::Plus),
      Some(b'-') => self.advance_and_build(Token::Dash),
      Some(b'*') => self.advance_and_build(Token::Star),
      Some(b'/') => {
        self.advance();
        match self.peek() {
          Some(b'/') => self.comment(),
          _ => self.build(Token::Slash),
        }
      }
      Some(b'>') => {
        self.advance();
        match self.peek() {
          Some(b'=') => self.advance_and_build(Token::GreaterEqual),
          _ => self.build(Token::Greater),
        }
      }
      Some(b'<') => {
        self.advance();
        match self.peek() {
          Some(b'=') => self.advance_and_build(Token::LessEqual),
          _ => self.build(Token::Less),
        }
      }
      Some(b'=') => {
        self.advance();
        match self.peek() {
          Some(b'=') => self.advance_and_build(Token::EqualEqual),
          _ => self.build(Token::Equal),
        }
      }
      Some(b'!') => {
        self.advance();
        match self.peek() {
          Some(b'=') => self.advance_and_build(Token::BangEqual),
          _ => self.build(Token::Bang),
        }
      }
      Some(b'&') => {
        self.advance();
        match self.peek() {
          Some(b'&') => self.advance_and_build(Token::AmpAmp),
          Some(byte) => self.build_err(LexError::UnexpectedChar(byte)),
          None => self.build_err(LexError::UnexpectedEof),
        }
      }
      Some(b'|') => {
        self.advance();
        match self.peek() {
          Some(b'|') => self.advance_and_build(Token::PipePipe),
          Some(byte) => self.build_err(LexError::UnexpectedChar(byte)),
          None => self.build_err(LexError::UnexpectedEof),
        }
      }
      Some(b',') => self.advance_and_build(Token::Comma),
      Some(b'.') => self.advance_and_build(Token::Period),
      Some(b':') => {
        self.advance();
        match self.peek() {
          Some(b':') => self.advance_and_build(Token::ColonColon),
          Some(byte) => self.build_err(LexError::UnexpectedChar(byte)),
          None => self.build_err(LexError::UnexpectedEof),
        }
      }
      Some(b';') => self.advance_and_build(Token::Semicolon),
      Some(b'(') => self.advance_and_build(Token::OpenParen),
      Some(b')') => self.advance_and_build(Token::CloseParen),
      Some(b'{') => self.advance_and_build(Token::OpenBrace),
      Some(b'}') => self.advance_and_build(Token::CloseBrace),
      Some(b'[') => self.advance_and_build(Token::OpenBracket),
      Some(b']') => self.advance_and_build(Token::CloseBracket),
      Some(byte) if is_digit(byte) => self.number(),
      Some(byte) if is_alphabetic(byte) => self.identifier(),
      Some(byte) => self.advance_and_build_err(LexError::UnexpectedChar(byte)),
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

  fn string(&mut self) -> Result<Spanned<Token>, Spanned<LexError>> {
    self.advance();
    loop {
      match self.peek() {
        Some(b'"') => break,
        None => return self.advance_and_build_err(LexError::UnexpectedEof),
        _ => self.advance(),
      };
    }
    self.advance();
    self.build(Token::String)
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

  fn identifier(&mut self) -> Result<Spanned<Token>, Spanned<LexError>> {
    loop {
      match self.peek() {
        Some(byte) if !is_digit(byte) && !is_alphabetic(byte) => break,
        None => break,
        _ => self.advance(),
      };
    }

    let token = self.build(Token::Identifier)?;
    match token.span().as_str() {
      "use" => Ok(token.map(|_| Token::Use)),
      "mod" => Ok(token.map(|_| Token::Mod)),
      "fn" => Ok(token.map(|_| Token::Fn)),
      "impl" => Ok(token.map(|_| Token::Impl)),
      "let" => Ok(token.map(|_| Token::Let)),
      "mut" => Ok(token.map(|_| Token::Mut)),
      "if" => Ok(token.map(|_| Token::If)),
      "else" => Ok(token.map(|_| Token::Else)),
      "while" => Ok(token.map(|_| Token::While)),
      "true" => Ok(token.map(|_| Token::True)),
      "false" => Ok(token.map(|_| Token::False)),
      _ => Ok(token),
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

  fn advance_and_build(
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

  fn advance_and_build_err(
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
}

fn is_digit(byte: u8) -> bool {
  byte >= b'0' && byte <= b'9'
}

fn is_whitespace(byte: u8) -> bool {
  byte == b'\r' || byte == b' ' || byte == b'\t' || byte == b'\n'
}

fn is_alphabetic(byte: u8) -> bool {
  byte >= b'a' && byte <= b'z' || byte >= b'A' && byte <= b'Z'
}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

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
    let lexer = Lexer::new("+ - * / > >= < <= = == ! != && ||");
    let source = lexer.source().clone();
    let tokens = lexer.collect();

    assert_eq!(
      tokens,
      vec![
        Ok(Spanned::new(Token::Plus, Span::new(source.clone(), 0, 1))),
        Ok(Spanned::new(Token::Dash, Span::new(source.clone(), 2, 3))),
        Ok(Spanned::new(Token::Star, Span::new(source.clone(), 4, 5))),
        Ok(Spanned::new(Token::Slash, Span::new(source.clone(), 6, 7))),
        Ok(Spanned::new(
          Token::Greater,
          Span::new(source.clone(), 8, 9)
        )),
        Ok(Spanned::new(
          Token::GreaterEqual,
          Span::new(source.clone(), 10, 12)
        )),
        Ok(Spanned::new(Token::Less, Span::new(source.clone(), 13, 14))),
        Ok(Spanned::new(
          Token::LessEqual,
          Span::new(source.clone(), 15, 17)
        )),
        Ok(Spanned::new(
          Token::Equal,
          Span::new(source.clone(), 18, 19)
        )),
        Ok(Spanned::new(
          Token::EqualEqual,
          Span::new(source.clone(), 20, 22)
        )),
        Ok(Spanned::new(Token::Bang, Span::new(source.clone(), 23, 24))),
        Ok(Spanned::new(
          Token::BangEqual,
          Span::new(source.clone(), 25, 27)
        )),
        Ok(Spanned::new(
          Token::AmpAmp,
          Span::new(source.clone(), 28, 30)
        )),
        Ok(Spanned::new(
          Token::PipePipe,
          Span::new(source.clone(), 31, 33)
        )),
        Ok(Spanned::new(Token::Eof, Span::new(source.clone(), 33, 33))),
      ]
    );
  }

  #[test]
  fn string() {
    let lexer = Lexer::new("\"foo\"");
    let source = lexer.source().clone();
    let tokens = lexer.collect();

    assert_eq!(
      tokens,
      vec![
        Ok(Spanned::new(Token::String, Span::new(source.clone(), 0, 5))),
        Ok(Spanned::new(Token::Eof, Span::new(source.clone(), 5, 5))),
      ]
    );
  }

  #[test]
  fn keywords() {
    let lexer = Lexer::new("true false fn mod impl let mut if else while");
    let source = lexer.source().clone();
    let tokens = lexer.collect();

    assert_eq!(
      tokens,
      vec![
        Ok(Spanned::new(Token::True, Span::new(source.clone(), 0, 4))),
        Ok(Spanned::new(Token::False, Span::new(source.clone(), 5, 10))),
        Ok(Spanned::new(Token::Fn, Span::new(source.clone(), 11, 13))),
        Ok(Spanned::new(Token::Mod, Span::new(source.clone(), 14, 17))),
        Ok(Spanned::new(Token::Impl, Span::new(source.clone(), 18, 22))),
        Ok(Spanned::new(Token::Let, Span::new(source.clone(), 23, 26))),
        Ok(Spanned::new(Token::Mut, Span::new(source.clone(), 27, 30))),
        Ok(Spanned::new(Token::If, Span::new(source.clone(), 31, 33))),
        Ok(Spanned::new(Token::Else, Span::new(source.clone(), 34, 38))),
        Ok(Spanned::new(Token::Else, Span::new(source.clone(), 39, 44))),
        Ok(Spanned::new(Token::Eof, Span::new(source.clone(), 44, 44))),
      ]
    );
  }

  #[test]
  fn identifier() {
    let lexer = Lexer::new("foo");
    let source = lexer.source().clone();
    let tokens = lexer.collect();

    assert_eq!(
      tokens,
      vec![
        Ok(Spanned::new(
          Token::Identifier,
          Span::new(source.clone(), 0, 3)
        )),
        Ok(Spanned::new(Token::Eof, Span::new(source.clone(), 3, 3))),
      ]
    );
  }
}

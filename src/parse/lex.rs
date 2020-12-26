use std::rc::Rc;

use crate::{
  common::{
    ident::Ident,
    source::Source,
    span::{Span, Spanned},
  },
  parse::{
    error::ParseError,
    token::{Keyword, Token},
  },
};

pub struct Lexer {
  source: Rc<Source>,
  offset: usize,
}

impl Lexer {
  pub fn new(source: &Rc<Source>) -> Lexer {
    Lexer {
      source: Rc::clone(source),
      offset: 0,
    }
  }

  pub fn is_done(&self) -> bool {
    // TODO: check for whitespace
    self.offset == self.source.contents().len()
  }

  pub fn next(&mut self) -> Result<Spanned<Token>, ParseError> {
    self.whitespace()?;

    match self.peek()?.item() {
      b'/' => return self.slash(),
      byte if is_digit(byte) => return self.int_or_float(),
      byte if is_ident_prefix(byte) => return self.ident_or_keyword(),
      _ => {}
    }

    Ok(self.advance()?.map(|byte| match byte {
      b'\n' => Token::Newline,
      b'+' => Token::Add,
      b'-' => Token::Sub,
      b'*' => Token::Mul,
      b'%' => Token::Rem,
      b'.' => Token::Dot,
      b'(' => Token::LParen,
      b')' => Token::RParen,
      b'{' => Token::LBrace,
      b'}' => Token::RBrace,
      b'[' => Token::LBracket,
      b']' => Token::RBracket,
      _ => unimplemented!(),
    }))
  }

  fn whitespace(&mut self) -> Result<(), ParseError> {
    while !self.is_done() {
      match self.peek()?.item() {
        b'\n' => break,
        byte if is_whitespace(byte) => {}
        _ => break,
      }
      self.advance()?;
    }
    Ok(())
  }

  fn slash(&mut self) -> Result<Spanned<Token>, ParseError> {
    let spanned = self.expect(b'/')?;

    match self.peek()?.item() {
      b'/' => {
        self.retreat();
        self.comment()
      }
      b'*' => {
        self.retreat();
        self.block_comment()
      }
      _ => Ok(spanned.map(|_| Token::Div)),
    }
  }

  fn int_or_float(&mut self) -> Result<Spanned<Token>, ParseError> {
    let offset = self.offset;

    let mut is_float = false;
    let mut bytes = Vec::new();
    while !self.is_done() {
      match self.peek()?.item() {
        b'.' if !is_float => is_float = true,
        b'.' => unimplemented!(),
        b'_' => {}
        byte if is_digit(byte) => {}
        _ => break,
      }
      bytes.push(*self.advance()?.item());
    }

    let len = bytes.len();
    let token = if is_float {
      Token::Float(String::from_utf8(bytes).unwrap().parse().unwrap())
    } else {
      Token::Int(String::from_utf8(bytes).unwrap().parse().unwrap())
    };

    Ok(Spanned::new(Span::new(&self.source, offset, len), token))
  }

  fn ident_or_keyword(&mut self) -> Result<Spanned<Token>, ParseError> {
    let offset = self.offset;

    let mut bytes = Vec::new();
    while !self.is_done() {
      match self.peek()?.item() {
        byte if is_ident(byte) => {}
        byte if is_ident_suffix(byte) => {
          bytes.push(*self.advance()?.item());
          if is_ident(self.peek()?.item()) {
            unimplemented!();
          }
          break;
        }
        _ => break,
      }
      bytes.push(*self.advance()?.item());
    }

    let len = bytes.len();
    let ident = String::from_utf8(bytes).unwrap();
    let token = match ident.as_str() {
      "mod" => Token::Keyword(Keyword::Mod),
      "fun" => Token::Keyword(Keyword::Fun),
      _ => Token::Ident(Ident::new(ident)),
    };

    Ok(Spanned::new(Span::new(&self.source, offset, len), token))
  }

  fn comment(&mut self) -> Result<Spanned<Token>, ParseError> {
    let offset = self.offset;

    let mut bytes = Vec::new();
    while !self.is_done() {
      let next_byte = *self.advance()?.item();
      bytes.push(next_byte);
      if next_byte == b'\n' {
        break;
      }
    }

    Ok(Spanned::new(
      Span::new(&self.source, offset, bytes.len()),
      Token::Comment(String::from_utf8(bytes).unwrap()),
    ))
  }

  fn block_comment(&mut self) -> Result<Spanned<Token>, ParseError> {
    let offset = self.offset;

    let mut bytes = Vec::new();
    loop {
      let next_bytes = (*self.advance()?.item(), *self.advance()?.item());
      bytes.push(next_bytes.0);
      bytes.push(next_bytes.1);
      if next_bytes == (b'*', b'/') {
        break;
      }
    }

    Ok(Spanned::new(
      Span::new(&self.source, offset, bytes.len()),
      Token::Comment(String::from_utf8(bytes).unwrap()),
    ))
  }

  fn peek(&self) -> Result<Spanned<u8>, ParseError> {
    self
      .source
      .contents()
      .as_bytes()
      .get(self.offset)
      .ok_or_else(|| unimplemented!())
      .map(|byte| Spanned::new(Span::new(&self.source, self.offset, 1), *byte))
  }

  fn advance(&mut self) -> Result<Spanned<u8>, ParseError> {
    let byte = self.peek()?;
    self.offset += 1;
    Ok(byte)
  }

  fn retreat(&mut self) {
    self.offset -= 1;
  }

  fn expect(&mut self, byte: u8) -> Result<Spanned<u8>, ParseError> {
    let spanned = self.advance()?;
    if spanned.item() == &byte {
      Ok(spanned)
    } else {
      unimplemented!();
    }
  }
}

fn is_ident(byte: &u8) -> bool {
  match byte {
    b'_' => true,
    byte if is_alpha(byte) => true,
    byte if is_digit(byte) => true,
    _ => false,
  }
}

fn is_ident_prefix(byte: &u8) -> bool {
  match byte {
    b'_' => true,
    byte if is_alpha(byte) => true,
    _ => false,
  }
}

fn is_ident_suffix(byte: &u8) -> bool {
  match byte {
    b'?' | b'!' => true,
    b'_' => true,
    byte if is_ident(byte) => true,
    _ => false,
  }
}

fn is_alpha(byte: &u8) -> bool {
  match byte {
    byte if byte >= &b'a' && byte <= &b'z' => true,
    byte if byte >= &b'A' && byte <= &b'Z' => true,
    _ => false,
  }
}

fn is_digit(byte: &u8) -> bool {
  match byte {
    byte if byte >= &b'0' && byte <= &b'9' => true,
    _ => false,
  }
}

fn is_whitespace(byte: &u8) -> bool {
  match byte {
    b'\n' | b'\r' | b'\t' | b' ' => true,
    _ => false,
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    common::{
      ident::Ident,
      source::Source,
      span::{Span, Spanned},
    },
    parse::token::{Keyword, Token},
  };

  use super::Lexer;

  #[test]
  fn newline() {
    let source = Source::new("\n \n \n".to_string());
    let mut lexer = Lexer::new(&source);
    assert_eq!(
      lexer.next().unwrap(),
      Spanned::new(Span::new(&source, 0, 1), Token::Newline)
    );
    assert_eq!(
      lexer.next().unwrap(),
      Spanned::new(Span::new(&source, 2, 1), Token::Newline)
    );
    assert_eq!(
      lexer.next().unwrap(),
      Spanned::new(Span::new(&source, 4, 1), Token::Newline)
    );
  }

  #[test]
  fn int() {
    let source = Source::new("123".to_string());
    let mut lexer = Lexer::new(&source);
    assert_eq!(
      lexer.next().unwrap(),
      Spanned::new(Span::new(&source, 0, 3), Token::Int(123))
    );
  }

  #[test]
  fn float() {
    let source = Source::new("12.3".to_string());
    let mut lexer = Lexer::new(&source);
    assert_eq!(
      lexer.next().unwrap(),
      Spanned::new(Span::new(&source, 0, 4), Token::Float(12.3))
    );
  }

  #[test]
  fn keyword() {
    let source = Source::new("mod fun".to_string());
    let mut lexer = Lexer::new(&source);
    assert_eq!(
      lexer.next().unwrap(),
      Spanned::new(Span::new(&source, 0, 3), Token::Keyword(Keyword::Mod))
    );
    assert_eq!(
      lexer.next().unwrap(),
      Spanned::new(Span::new(&source, 4, 3), Token::Keyword(Keyword::Fun))
    );
  }

  #[test]
  fn ident() {
    let source = Source::new("foo bar".to_string());
    let mut lexer = Lexer::new(&source);
    assert_eq!(
      lexer.next().unwrap(),
      Spanned::new(
        Span::new(&source, 0, 3),
        Token::Ident(Ident::new("foo".to_string()))
      )
    );
    assert_eq!(
      lexer.next().unwrap(),
      Spanned::new(
        Span::new(&source, 4, 3),
        Token::Ident(Ident::new("bar".to_string()))
      )
    );
  }

  #[test]
  fn comment() {
    let source = Source::new(
      "// This is a comment.\n// This is another comment.".to_string(),
    );
    let mut lexer = Lexer::new(&source);
    assert_eq!(
      lexer.next().unwrap(),
      Spanned::new(
        Span::new(&source, 0, 22),
        Token::Comment("// This is a comment.\n".to_string())
      )
    );
    assert_eq!(
      lexer.next().unwrap(),
      Spanned::new(
        Span::new(&source, 22, 27),
        Token::Comment("// This is another comment.".to_string())
      )
    );
  }

  #[test]
  fn block_comment() {
    let source = Source::new(
      "/* This is a comment. *//* This is another comment. */".to_string(),
    );
    let mut lexer = Lexer::new(&source);
    assert_eq!(
      lexer.next().unwrap(),
      Spanned::new(
        Span::new(&source, 0, 24),
        Token::Comment("/* This is a comment. */".to_string())
      )
    );
    assert_eq!(
      lexer.next().unwrap(),
      Spanned::new(
        Span::new(&source, 24, 30),
        Token::Comment("/* This is another comment. */".to_string())
      )
    );
  }
}

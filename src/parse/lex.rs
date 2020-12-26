use std::rc::Rc;

use crate::{
  common::{
    source::Source,
    span::{Span, Spanned},
  },
  parse::{error::ParseError, token::Token},
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
    self.offset == self.source.contents().len()
  }

  pub fn next(&mut self) -> Result<Spanned<Token>, ParseError> {
    match self.peek()?.item() {
      b'/' => return self.slash(),
      _ => {}
    }

    Ok(self.advance()?.map(|byte| match byte {
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

  fn slash(&mut self) -> Result<Spanned<Token>, ParseError> {
    let spanned = self.expect(b'/')?;

    match self.peek()?.item() {
      b'/' => {
        self.retreat();
        self.comment()
      }
      _ => Ok(spanned.map(|_| Token::Div)),
    }
  }

  fn comment(&mut self) -> Result<Spanned<Token>, ParseError> {
    let offset = self.offset;

    let mut bytes = Vec::new();
    while !self.is_done() {
      let byte = *self.advance()?.item();
      bytes.push(byte);
      if byte == b'\n' {
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

#[cfg(test)]
mod tests {
  use crate::{
    common::{
      source::Source,
      span::{Span, Spanned},
    },
    parse::token::Token,
  };

  use super::Lexer;

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
}

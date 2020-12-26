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

  pub fn next(&mut self) -> Result<Spanned<Token>, ParseError> {
    self.whitespace()?;

    if self.is_done() {
      return Ok(Spanned::new(
        Span::new(&self.source, self.offset, 0),
        Token::Eof,
      ));
    }

    match self.peek()?.item() {
      b'/' => return self.div_or_comment(),
      b'&' => return self.and(),
      b'|' => return self.or_or_pipe(),
      b'>' => return self.gt_or_gte(),
      b'<' => return self.lt_or_lte(),
      b'=' => return self.ass_or_eq(),
      b'!' => return self.not_or_neq(),
      byte if is_digit(byte) => return self.int_or_float(),
      byte if is_ident_prefix(byte) => return self.ident_or_keyword(),
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
      b'\n' => Token::Newline,
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

  fn div_or_comment(&mut self) -> Result<Spanned<Token>, ParseError> {
    let div = self.expect(b'/')?;

    match self.peek()?.item() {
      b'/' => {
        self.retreat();
        self.comment()
      }
      b'*' => {
        self.retreat();
        self.block_comment()
      }
      _ => Ok(div.map(|_| Token::Div)),
    }
  }

  fn and(&mut self) -> Result<Spanned<Token>, ParseError> {
    self.two_byte_op((b'&', b'&'), Token::And)
  }

  fn or_or_pipe(&mut self) -> Result<Spanned<Token>, ParseError> {
    self.expect(b'|')?;

    match self.peek()?.item() {
      b'|' => {
        self.retreat();
        self.two_byte_op((b'|', b'|'), Token::Or)
      }
      b'>' => {
        self.retreat();
        self.two_byte_op((b'|', b'>'), Token::Pipe)
      }
      _ => unimplemented!(),
    }
  }

  fn gt_or_gte(&mut self) -> Result<Spanned<Token>, ParseError> {
    self.one_or_two_byte_op((b'>', b'='), Token::Gt, Token::Gte)
  }

  fn lt_or_lte(&mut self) -> Result<Spanned<Token>, ParseError> {
    self.one_or_two_byte_op((b'<', b'='), Token::Lt, Token::Lte)
  }

  fn ass_or_eq(&mut self) -> Result<Spanned<Token>, ParseError> {
    self.one_or_two_byte_op((b'=', b'='), Token::Ass, Token::Eq)
  }

  fn not_or_neq(&mut self) -> Result<Spanned<Token>, ParseError> {
    self.one_or_two_byte_op((b'!', b'='), Token::Not, Token::Neq)
  }

  fn one_or_two_byte_op(
    &mut self,
    bytes: (u8, u8),
    one_byte_token: Token,
    two_byte_token: Token,
  ) -> Result<Spanned<Token>, ParseError> {
    let first = self.expect(bytes.0)?;

    if self.is_done() {
      return Ok(first.map(|_| one_byte_token.clone()));
    }

    match self.peek()?.item() {
      byte if byte == &bytes.1 => {
        self.retreat();
        self.two_byte_op((bytes.0, bytes.1), two_byte_token)
      }
      _ => Ok(first.map(|_| one_byte_token.clone())),
    }
  }

  fn two_byte_op(
    &mut self,
    bytes: (u8, u8),
    token: Token,
  ) -> Result<Spanned<Token>, ParseError> {
    let first = self.expect(bytes.0)?;
    let second = self.expect(bytes.1)?;

    Ok(Spanned::new(
      Span::combine(first.span(), second.span()),
      token,
    ))
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
    let int_or_float = String::from_utf8(bytes).unwrap();
    let token = if is_float {
      Token::Float(int_or_float.parse().unwrap())
    } else {
      Token::Int(int_or_float.parse().unwrap())
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
      let next_bytes = (*self.advance()?.item(), *self.peek()?.item());
      bytes.push(next_bytes.0);
      if next_bytes == (b'*', b'/') {
        bytes.push(*self.advance()?.item());
        break;
      }
    }

    Ok(Spanned::new(
      Span::new(&self.source, offset, bytes.len()),
      Token::Comment(String::from_utf8(bytes).unwrap()),
    ))
  }

  fn is_done(&self) -> bool {
    self.offset == self.source.contents().len()
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
  fn eof() {
    let source = Source::new("    ".to_string());
    let mut lexer = Lexer::new(&source);
    assert_eq!(
      lexer.next().unwrap(),
      Spanned::new(Span::new(&source, 4, 0), Token::Eof)
    );
  }

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
  fn ops() {
    let source =
      Source::new("+ - * / % . |> && || > >= < <= == != ! =".to_string());
    let mut lexer = Lexer::new(&source);
    let spanneds = vec![
      Spanned::new(Span::new(&source, 0, 1), Token::Add),
      Spanned::new(Span::new(&source, 2, 1), Token::Sub),
      Spanned::new(Span::new(&source, 4, 1), Token::Mul),
      Spanned::new(Span::new(&source, 6, 1), Token::Div),
      Spanned::new(Span::new(&source, 8, 1), Token::Rem),
      Spanned::new(Span::new(&source, 10, 1), Token::Dot),
      Spanned::new(Span::new(&source, 12, 2), Token::Pipe),
      Spanned::new(Span::new(&source, 15, 2), Token::And),
      Spanned::new(Span::new(&source, 18, 2), Token::Or),
      Spanned::new(Span::new(&source, 21, 1), Token::Gt),
      Spanned::new(Span::new(&source, 23, 2), Token::Gte),
      Spanned::new(Span::new(&source, 26, 1), Token::Lt),
      Spanned::new(Span::new(&source, 28, 2), Token::Lte),
      Spanned::new(Span::new(&source, 31, 2), Token::Eq),
      Spanned::new(Span::new(&source, 34, 2), Token::Neq),
      Spanned::new(Span::new(&source, 37, 1), Token::Not),
      Spanned::new(Span::new(&source, 39, 1), Token::Ass),
    ];
    for spanned in spanneds {
      assert_eq!(lexer.next().unwrap(), spanned);
    }
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

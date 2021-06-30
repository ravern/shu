use std::{
  cmp::{max, min},
  iter::Peekable,
};

pub struct Spanned<T> {
  span: Span,
  base: T,
}

impl<T> Spanned<T> {
  pub fn span(&self) -> Span {
    self.span
  }

  pub fn base(&self) -> &T {
    &self.base
  }
}

#[derive(Clone, Copy)]
pub struct Span {
  start: usize,
  end: usize,
}

impl Span {
  pub fn new(start: usize, end: usize) -> Span {
    if start >= end {
      panic!("start of span must be before end");
    }
    Span { start, end }
  }

  pub fn combine(&self, other: Span) -> Span {
    Span {
      start: min(self.start, other.start),
      end: max(self.end, other.end),
    }
  }
}

pub enum Token {
  // Literals
  Int(i64),
  Float(f64),
  Bool(bool),
  String(String),
  Ident(String),
  // Keywords
  Mod,
  Use,
  Fn,
  Struct,
  Enum,
  Impl,
  Let,
  If,
  For,
  While,
  Loop,
  Match,
  // Punctuation
  Gt,
  Lt,
  GtEq,
  LtEq,
  Eq,
  EqEq,
  LParen,
  RParen,
  LBrace,
  RBrace,
  LBracket,
  RBracket,
  Comma,
  Semi,
  Col,
  ColCol,
  // Misc.
  Comment(String),
  Eof,
}

pub enum LexError {}

pub struct Lexer<S>
where
  S: Iterator<Item = Spanned<char>>,
{
  source: Peekable<S>,
}

impl<S> Lexer<S>
where
  S: Iterator<Item = Spanned<char>>,
{
  pub fn new(source: S) -> Lexer<S> {
    Lexer {
      source: source.peekable(),
    }
  }

  pub fn next_token(&mut self) -> Option<Token> {
    None
  }
}

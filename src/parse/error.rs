use std::fmt;

use thiserror::Error;

use crate::common::span::Span;

#[derive(Debug, Error)]
pub struct ParseError {
  span: Span,
  unexpected: Expected,
  expected: Option<Vec<Expected>>,
}

impl ParseError {
  pub fn new(span: Span, unexpected: Expected) -> ParseError {
    ParseError {
      span,
      unexpected,
      expected: None,
    }
  }
}

impl fmt::Display for ParseError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    write!(fmt, "test")
  }
}

#[derive(Debug)]
pub enum Expected {
  Byte(u8),
  Eof,
}

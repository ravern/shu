use std::iter::Peekable;

use crate::{
  common::span::{Span, Spanned},
  parse::token::Token,
};

pub struct Lexer<S>
where
  S: Iterator<Item = char>,
{
  source: Peekable<S>,
}

impl<S> Lexer<S>
where
  S: Iterator<Item = char>,
{
  pub fn new(source: S) -> Lexer<S> {
    Lexer {
      source: source.peekable(),
    }
  }

  pub fn next(&mut self) -> Option<Result<Spanned<Token>, LexError>> {
    None
  }
}

pub enum LexError {}

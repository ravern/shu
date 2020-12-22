use crate::{ast::ModBody, common::span::Spanned};

mod lexer;
mod token;

pub fn parse<S>(source: S) -> Result<Spanned<ModBody>, ()> where S: Iterator<Item = char> {
  Err(())
}

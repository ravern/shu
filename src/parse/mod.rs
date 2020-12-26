use std::{fs::File, path::Path};

use crate::{ast::ModBody, common::span::Spanned};

pub use self::error::ParseError;

mod error;
mod lex;
mod token;

pub fn parse_path<P>(path: P) -> Result<Spanned<ModBody>, ParseError>
where
  P: AsRef<Path>,
{
  Err(ParseError::Syntax(error::SyntaxError {}))
}

pub fn parse_file(file: File) -> Result<Spanned<ModBody>, ParseError> {
  Err(ParseError::Syntax(error::SyntaxError {}))
}

pub fn parse_str<S>(string: S) -> Result<Spanned<ModBody>, ParseError>
where
  S: AsRef<str>,
{
  Err(ParseError::Syntax(error::SyntaxError {}))
}

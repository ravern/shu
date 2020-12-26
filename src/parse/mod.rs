use std::{fs::File, path::Path};

use crate::{ast::ModBody, common::span::Spanned, parse::error::ParseError};

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

pub fn parse_str<S>(str: S) -> Result<Spanned<ModBody>, ParseError>
where
  S: AsRef<str>,
{
  Err(ParseError::Syntax(error::SyntaxError {}))
}

#[cfg(test)]
mod tests {
  use super::parse_str;

  #[test]
  fn fn_def() {
    parse_str(
      "fun fibonacci(n) {
        if n < 2 {
          1
        } else {
          fibonacci(n - 2) + fibonacci(n - 1)
        }
      }",
    );
  }
}

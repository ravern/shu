use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
  #[error("{0}")]
  Syntax(#[from] SyntaxError),
}

#[derive(Debug, Error)]
#[error("invalid syntax")]
pub struct SyntaxError {}

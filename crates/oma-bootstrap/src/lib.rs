use crate::{expand::ExpandError, parse::ParseError};

mod ast;
mod compile;
mod expand;
mod parse;

pub enum Error {
  Parse(ParseError),
  Expand(ExpandError),
}

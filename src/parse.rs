use crate::{
  ast::{File, Stmt},
  token::Token,
};

pub fn parse(tokens: Vec<Token>) -> Result<File, ParseError> {
  let mut parser = Parser::new(tokens);
  parser.parse_file()
}

pub struct Parser {
  tokens: Vec<Token>,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Parser {
    Parser { tokens }
  }

  pub fn parse_file(&mut self) -> Result<File, ParseError> {
    Ok(File(vec![]))
  }
}

#[derive(Debug)]
pub enum ParseError {}

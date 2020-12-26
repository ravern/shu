use crate::common::ident::Ident;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  Int(i64),
  Float(f64),
  Bool(bool),
  String(String),
  Ident(Ident),
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Dot,
  Pipe,
  And,
  Or,
  Gt,
  Gte,
  Lt,
  Lte,
  Eq,
  Neq,
  Not,
  Ass,
  LParen,
  RParen,
  LBrace,
  RBrace,
  LBracket,
  RBracket,
  Comment(String),
  Keyword(Keyword),
  Newline,
  Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
  Mod,
  Fun,
}

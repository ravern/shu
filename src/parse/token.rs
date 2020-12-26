use crate::common::ident::Ident;

#[derive(Debug, PartialEq)]
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
  And,
  Or,
  Gt,
  Gte,
  Lt,
  Lte,
  Eq,
  Neq,
  Not,
  Assign,
  LParen,
  RParen,
  LBrace,
  RBrace,
  LBracket,
  RBracket,
  Comment(String),
  Keyword(Keyword),
  Newline,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
  Mod,
  Fun,
}

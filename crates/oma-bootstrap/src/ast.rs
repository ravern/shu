use crate::lex::{Spanned, Token};

pub enum Stmt {
  Use(Use),
  Mod(Mod),
  Fn(Fn),
}

pub struct Use {}

pub struct Mod {
  body: Vec<Stmt>,
}

pub struct Fn {
  params: Vec<Spanned<Token>>,
}

pub struct Block {
  exprs: Vec<Expr>,
}

pub enum Expr {
  If(If),
  Match(Match),
}

pub struct If {
  if_body: Vec<Expr>,
  else_body: Else,
}

pub enum Else {
  Else(Vec<Expr>),
  ElseIf(Box<If>),
}

pub struct Match {
  subject: Box<Expr>,
  arms: Vec<Arm>,
}

pub struct Arm {
  pat: Pat,
  body: Vec<Expr>,
}

pub enum Pat {}

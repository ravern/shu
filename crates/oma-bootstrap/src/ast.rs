use crate::{span::Spanned, token::Token};

#[derive(Debug)]
pub enum Expression {
  Literal(LiteralExpression),
  Unary(UnaryExpression),
  Binary(BinaryExpression),
}

#[derive(Debug)]
pub struct UnaryExpression {
  pub operator: Spanned<Token>,
  pub operand: Box<Spanned<Expression>>,
}

#[derive(Debug)]
pub struct BinaryExpression {
  pub operator: Spanned<Token>,
  pub left_operand: Box<Spanned<Expression>>,
  pub right_operand: Box<Spanned<Expression>>,
}

#[derive(Debug)]
pub enum LiteralExpression {
  Int(i64),
  Float(f64),
}

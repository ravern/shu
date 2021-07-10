use crate::{span::Spanned, token::Token};

#[derive(Debug, PartialEq)]
pub enum Expression {
  Literal(LiteralExpression),
  Unary(UnaryExpression),
  Binary(BinaryExpression),
}

impl Expression {
  pub fn binary(
    operator: Spanned<Token>,
    left_operand: Spanned<Expression>,
    right_operand: Spanned<Expression>,
  ) -> Expression {
    Expression::Binary(BinaryExpression {
      operator,
      left_operand: Box::new(left_operand),
      right_operand: Box::new(right_operand),
    })
  }

  pub fn unary(
    operator: Spanned<Token>,
    operand: Spanned<Expression>,
  ) -> Expression {
    Expression::Unary(UnaryExpression {
      operator,
      operand: Box::new(operand),
    })
  }

  pub fn int(int: i64) -> Expression {
    Expression::Literal(LiteralExpression::Int(int))
  }

  pub fn float(float: f64) -> Expression {
    Expression::Literal(LiteralExpression::Float(float))
  }

  pub fn bool(bool: bool) -> Expression {
    Expression::Literal(LiteralExpression::Bool(bool))
  }
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpression {
  pub operator: Spanned<Token>,
  pub operand: Box<Spanned<Expression>>,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression {
  pub operator: Spanned<Token>,
  pub left_operand: Box<Spanned<Expression>>,
  pub right_operand: Box<Spanned<Expression>>,
}

#[derive(Debug, PartialEq)]
pub enum LiteralExpression {
  Int(i64),
  Float(f64),
  Bool(bool),
}

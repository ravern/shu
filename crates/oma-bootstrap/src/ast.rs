use crate::{span::Spanned, token::Token};

#[derive(Debug, PartialEq)]
pub struct Block {
  pub open_brace_token: Spanned<Token>,
  pub statements: Vec<Spanned<Statement>>,
  pub close_brace_token: Spanned<Token>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
  Bind(BindStatement),
  Expression(ExpressionStatement),
}

impl Statement {
  pub fn bind(
    let_token: Spanned<Token>,
    mut_token: Option<Spanned<Token>>,
    pattern: Spanned<Expression>,
    equal_token: Spanned<Token>,
    expression: Spanned<Expression>,
    semicolon_token: Spanned<Token>,
  ) -> Statement {
    Statement::Bind(BindStatement {
      let_token,
      mut_token,
      pattern: Box::new(pattern),
      equal_token,
      expression: Box::new(expression),
      semicolon_token,
    })
  }

  pub fn expression(
    expression: Spanned<Expression>,
    semicolon_token: Option<Spanned<Token>>,
  ) -> Statement {
    Statement::Expression(ExpressionStatement {
      expression,
      semicolon_token,
    })
  }
}

#[derive(Debug, PartialEq)]
pub struct BindStatement {
  pub let_token: Spanned<Token>,
  pub mut_token: Option<Spanned<Token>>,
  pub pattern: Box<Spanned<Expression>>,
  pub equal_token: Spanned<Token>,
  pub expression: Box<Spanned<Expression>>,
  pub semicolon_token: Spanned<Token>,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
  pub expression: Spanned<Expression>,
  pub semicolon_token: Option<Spanned<Token>>,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
  Literal(LiteralExpression),
  Unary(UnaryExpression),
  Binary(BinaryExpression),
}

impl Expression {
  pub fn int(int: i64) -> Expression {
    Expression::Literal(LiteralExpression::Int(int))
  }

  pub fn float(float: f64) -> Expression {
    Expression::Literal(LiteralExpression::Float(float))
  }

  pub fn bool(bool: bool) -> Expression {
    Expression::Literal(LiteralExpression::Bool(bool))
  }

  pub fn identifier(identifier: String) -> Expression {
    Expression::Literal(LiteralExpression::Identifier(identifier))
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
}

// TODO: Rename to `operator_token`.
#[derive(Debug, PartialEq)]
pub struct UnaryExpression {
  pub operator: Spanned<Token>,
  pub operand: Box<Spanned<Expression>>,
}

// TODO: Rename to `operator_token`.
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
  Identifier(String),
}

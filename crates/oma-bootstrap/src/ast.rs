use crate::{lex::LexError, span::Spanned, token::Token};

#[derive(Debug, PartialEq)]
pub enum ParseError {
  Lex(LexError),
  UnexpectedToken(Token),
}

#[derive(Debug, PartialEq)]
pub struct File {
  pub declarations: Vec<Spanned<Declaration>>,
  pub eof_token: Spanned<Token>,
}

#[derive(Debug, PartialEq)]
pub enum Declaration {
  Use(UseDeclaration),
  Mod(ModDeclaration),
  Fn(FnDeclaration),
  Err(ParseError),
}

#[derive(Debug, PartialEq)]
pub struct UseDeclaration {
  pub use_token: Spanned<Token>,
  pub path_tree: Spanned<UseTree>,
}

#[derive(Debug, PartialEq)]
pub enum UseTree {
  Branch(UseTreeBranch),
  Leaf(Path),
}

#[derive(Debug, PartialEq)]
pub struct UseTreeBranch {
  prefix: Spanned<PathComponent>,
  children: Vec<Spanned<UseTreeBranchChild>>,
}

#[derive(Debug, PartialEq)]
pub struct UseTreeBranchChild {
  tree: Spanned<UseTree>,
  comma_token: Option<Spanned<Token>>,
}

#[derive(Debug, PartialEq)]
pub struct Path {
  components: Vec<Spanned<PathComponent>>,
}

#[derive(Debug, PartialEq)]
pub struct PathComponent {
  name: Spanned<Token>,
  separator_token: Option<Spanned<Token>>,
}

#[derive(Debug, PartialEq)]
pub struct ModDeclaration {
  pub mod_token: Spanned<Token>,
  pub name: Spanned<Token>,
  pub body: Option<ModBody>,
}

#[derive(Debug, PartialEq)]
pub struct ModBody {
  open_brace_token: Spanned<Token>,
  declarations: Vec<Spanned<Declaration>>,
  close_brace_token: Spanned<Token>,
}

#[derive(Debug, PartialEq)]
pub struct FnDeclaration {
  pub fn_token: Spanned<Token>,
  pub name: Spanned<Token>,
  pub parameters: Spanned<FnParameters>,
  pub body: Spanned<Block>,
}

#[derive(Debug, PartialEq)]
pub struct FnParameters {
  pub open_paren_token: Spanned<Token>,
  pub parameters: Vec<Spanned<FnParameter>>,
  pub close_paren_token: Spanned<Token>,
}

#[derive(Debug, PartialEq)]
pub struct FnParameter {
  pub name: Spanned<Token>,
  pub comma_token: Option<Spanned<Token>>,
}

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
    pattern: Spanned<Pattern>,
    equal_token: Spanned<Token>,
    expression: Spanned<Expression>,
    semicolon_token: Spanned<Token>,
  ) -> Statement {
    Statement::Bind(BindStatement {
      let_token,
      mut_token,
      pattern,
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
  pub pattern: Spanned<Pattern>,
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
  If(IfExpression),
  While(WhileExpression),
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
    operator_token: Spanned<Token>,
    operand: Spanned<Expression>,
  ) -> Expression {
    Expression::Unary(UnaryExpression {
      operator_token,
      operand: Box::new(operand),
    })
  }

  pub fn binary(
    operator_token: Spanned<Token>,
    left_operand: Spanned<Expression>,
    right_operand: Spanned<Expression>,
  ) -> Expression {
    Expression::Binary(BinaryExpression {
      operator_token,
      left_operand: Box::new(left_operand),
      right_operand: Box::new(right_operand),
    })
  }
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpression {
  pub operator_token: Spanned<Token>,
  pub operand: Box<Spanned<Expression>>,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression {
  pub left_operand: Box<Spanned<Expression>>,
  pub operator_token: Spanned<Token>,
  pub right_operand: Box<Spanned<Expression>>,
}

#[derive(Debug, PartialEq)]
pub enum LiteralExpression {
  Int(i64),
  Float(f64),
  Bool(bool),
  Identifier(String),
}

#[derive(Debug, PartialEq)]
pub struct IfExpression {
  pub if_token: Spanned<Token>,
  pub condition: Box<Spanned<Expression>>,
  pub body: Spanned<Block>,
  pub else_body: Option<Spanned<ElseBody>>,
}

#[derive(Debug, PartialEq)]
pub struct ElseBody {
  pub else_token: Spanned<Token>,
  pub block: Box<Spanned<ElseBlock>>,
}

#[derive(Debug, PartialEq)]
pub enum ElseBlock {
  Else(Block),
  If(IfExpression),
}

#[derive(Debug, PartialEq)]
pub struct WhileExpression {
  pub while_token: Spanned<Token>,
  pub condition: Box<Spanned<Expression>>,
  pub body: Spanned<Block>,
}

#[derive(Debug, PartialEq)]
pub enum Pattern {
  Int(i64),
  Float(f64),
  Bool(bool),
  Identifier(String),
}

use crate::{span::Spanned, token::Token};

#[derive(Debug, PartialEq)]
pub struct File {
  pub declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq)]
pub enum Declaration {
  Use(UseDeclaration),
  Mod(ModDeclaration),
  Fn(FnDeclaration),
}

#[derive(Debug, PartialEq)]
pub struct UseDeclaration {
  pub trees: Vec<UseTree>,
}

#[derive(Debug, PartialEq)]
pub enum UseTree {
  Branch(UseTreeBranch),
  Leaf(Spanned<Token>),
}

#[derive(Debug, PartialEq)]
pub struct UseTreeBranch {
  pub component: Spanned<Token>,
  pub subtrees: Vec<UseTree>,
}

#[derive(Debug, PartialEq)]
pub struct ModDeclaration {
  pub name: Spanned<Token>,
  pub body: Option<File>,
}

#[derive(Debug, PartialEq)]
pub struct FnDeclaration {
  pub name: Spanned<Token>,
  pub parameters: Vec<Spanned<Token>>,
  pub body: Block,
}

#[derive(Debug, PartialEq)]
pub struct Block {
  pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
  Bind(BindStatement),
  Expression(ExpressionStatement),
}

#[derive(Debug, PartialEq)]
pub struct BindStatement {
  pub is_mut: bool,
  pub pattern: Pattern,
  pub expression: Expression,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
  pub expression: Expression,
  pub has_semicolon: bool,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
  Literal(Spanned<Token>),
  Path(Path),
  Access(AccessExpression),
  Call(CallExpression),
  Unary(UnaryExpression),
  Binary(BinaryExpression),
  Assign(AssignExpression),
  If(IfExpression),
  While(WhileExpression),
}

#[derive(Debug, PartialEq)]
pub struct AccessExpression {
  pub receiver: Box<Expression>,
  pub field: Spanned<Token>,
}

#[derive(Debug, PartialEq)]
pub struct CallExpression {
  pub receiver: Box<Expression>,
  pub arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpression {
  pub operator: Spanned<Token>,
  pub operand: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression {
  pub left_operand: Box<Expression>,
  pub operator: Spanned<Token>,
  pub right_operand: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct AssignExpression {
  pub pattern: Pattern,
  pub operand: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct IfExpression {
  pub condition: Box<Expression>,
  pub body: Block,
  pub else_body: Option<ElseBody>,
}

#[derive(Debug, PartialEq)]
pub enum ElseBody {
  Else(Block),
  If(Box<IfExpression>),
}

#[derive(Debug, PartialEq)]
pub struct WhileExpression {
  pub condition: Box<Expression>,
  pub body: Block,
}

#[derive(Debug, PartialEq)]
pub struct Path {
  pub components: Vec<Spanned<Token>>,
}

#[derive(Debug, PartialEq)]
pub enum Pattern {
  Literal(Spanned<Token>),
  __NonExhaustive,
}

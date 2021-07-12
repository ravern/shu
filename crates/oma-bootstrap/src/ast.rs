use crate::{lex::LexError, span::Spanned, token::Token};

#[derive(Debug, PartialEq)]
pub enum ParseError {
  Lex(LexError),
  UnexpectedToken(Token),
}

#[derive(Debug, PartialEq)]
pub struct File {
  pub mod_declarations: Vec<ModDeclaration>,
  pub use_declarations: Vec<UseDeclaration>,
  pub fn_declarations: Vec<FnDeclaration>,
}

impl File {
  pub fn new() -> File {
    File {
      mod_declarations: Vec::new(),
      use_declarations: Vec::new(),
      fn_declarations: Vec::new(),
    }
  }
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
pub struct Path {
  components: Vec<Spanned<Token>>,
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
  pub expression: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
  pub expression: Expression,
  pub has_semicolon: bool,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
  Literal(Spanned<Token>),
  Unary(UnaryExpression),
  Binary(BinaryExpression),
  If(IfExpression),
  While(WhileExpression),
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
pub enum Pattern {
  Int(i64),
  Float(f64),
  Bool(bool),
  Identifier(String),
}

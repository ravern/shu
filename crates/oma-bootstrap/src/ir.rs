use std::collections::HashMap;

#[derive(Debug)]
pub struct Executable {
  pub package_header: PackageHeader,
  pub identifiers: HashMap<String, usize>,
  pub chunks: Vec<Chunk>,
}

#[derive(Debug)]
pub struct PackageHeader {
  pub package_headers: HashMap<usize, PackageHeader>,
  pub use_declarations: Vec<Path>,
  pub mod_headers: HashMap<usize, ModHeader>,
  pub fn_headers: HashMap<usize, usize>,
}

#[derive(Debug)]
pub struct ModHeader {
  pub use_declarations: Vec<Path>,
  pub mod_headers: HashMap<usize, ModHeader>,
  pub fn_headers: HashMap<usize, usize>,
}

#[derive(Debug)]
pub struct Chunk {
  pub parameters: Vec<usize>,
  pub body: Block,
}

#[derive(Debug)]
pub struct Block {
  pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
  Bind(BindStatement),
  Expression(ExpressionStatement),
}

#[derive(Debug)]
pub struct BindStatement {
  pub is_mut: bool,
  pub name: usize,
  pub expression: Expression,
}

#[derive(Debug)]
pub struct ExpressionStatement {
  pub expression: Expression,
}

#[derive(Debug)]
pub enum Expression {
  Block(Block),
  Literal(LiteralExpression),
  Path(Path),
  Access(AccessExpression),
  Call(CallExpression),
  Unary(UnaryExpression),
  Binary(BinaryExpression),
  If(IfExpression),
  While(WhileExpression),
}

#[derive(Debug)]
pub enum LiteralExpression {
  Int(i64),
  Float(i64),
  Bool(bool),
  Identifier(usize),
}

#[derive(Debug)]
pub struct AccessExpression {
  pub receiver: Box<Expression>,
  pub field: usize,
}

#[derive(Debug)]
pub struct CallExpression {
  pub receiver: Box<Expression>,
  pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub struct UnaryExpression {
  pub operator: UnaryOperator,
  pub operand: Box<Expression>,
}

#[derive(Debug)]
pub enum UnaryOperator {
  Negate,
  Not,
}

#[derive(Debug)]
pub struct BinaryExpression {
  pub left_operand: Box<Expression>,
  pub operator: BinaryOperator,
  pub right_operand: Box<Expression>,
}

#[derive(Debug)]
pub enum BinaryOperator {
  And,
  Or,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  Equal,
  Add,
  Subtract,
  Multiply,
  Divide,
}

#[derive(Debug)]
pub struct IfExpression {
  pub condition: Box<Expression>,
  pub body: Block,
  pub else_body: Option<ElseBody>,
}

#[derive(Debug)]
pub enum ElseBody {
  Else(Block),
  If(Box<IfExpression>),
}

#[derive(Debug)]
pub struct WhileExpression {
  pub condition: Box<Expression>,
  pub body: Block,
}

#[derive(Debug)]
pub struct Path {
  pub components: Vec<usize>,
}

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
  Literal(LiteralExpression),
  Path(Path),
}

#[derive(Debug)]
pub enum LiteralExpression {
  Identifier(usize),
}

#[derive(Debug)]
pub struct Path {
  pub components: Vec<usize>,
}

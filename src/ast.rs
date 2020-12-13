use std::rc::Rc;

use internment::Intern;

pub enum Expression {
  Block(Block),
  Literal(Literal),
  Identifier(Identifier),
  Tuple(Tuple),
  Record(Record),
  Type(Identifiers, Rc<Expression>),
  Function(Function),
  Module(Module),
  Unary(Operator, Rc<Expression>),
  Binary(Operator, Rc<Expression>, Rc<Expression>),
  Assign(Pattern, Rc<Expression>),
  Call(Rc<Expression>, Vec<Expression>),
  // Use(Use),
  If(If),
  Cond(Cond),
  Match(Match),
}

pub enum Pattern {
  Identifier(Identifier),
}

pub struct Block(Vec<Expression>);

pub struct Tuple(Vec<Expression>);

pub struct Record(Vec<(Identifier, Expression)>);

pub struct Function {
  name: Identifier,
  parameters: Vec<Identifier>,
  body: Block,
  public: bool,
}

impl Function {
  pub fn arity(&self) -> usize {
    self.parameters.len()
  }
}

pub struct Module {
  name: Identifier,
  body: Option<Block>,
  public: bool,
}

pub struct If {
  condition: Rc<Expression>,
  body: Block,
  els: Option<Else>,
}

pub enum Else {
  Block(Block),
  If(Rc<If>),
}

pub struct Cond {
  arms: Vec<(Expression, Expression)>,
}

pub struct Match {
  head: Rc<Expression>,
  arms: Vec<(Pattern, Expression)>,
}

pub enum Literal {
  Int(i64),
  Float(f64),
  String(String),
  Bool(bool),
  Unit,
}

pub struct Identifiers(Vec<Identifier>);

pub struct Identifier(Intern<String>);

impl Identifier {
  pub fn new<S>(identifier: S) -> Identifier
  where
    S: Into<String>,
  {
    Identifier(Intern::new(identifier.into()))
  }
}

pub enum Operator {
  Add,
  Subtract,
  Muliply,
  Divide,
  Mod,
  Equal,
  Greater,
  Less,
  GreaterEqual,
  LessEqual,
  And,
  Or,
  Not,
  Pipe,
}

use internment::Intern;

use crate::common::span::Spanned;

pub enum Stmt {
  Use(UseTree),
  ModDef(ModDef),
  FnDef(FnDef),
}

pub enum Expr {
  Lit(Lit),
  Tuple(Vec<Spanned<Expr>>),
  Record(Vec<(Spanned<Ident>, Spanned<Expr>)>),
  Label(Label),
  Assign(Assign),
  BinOp(BinOp),
  UnOp(UnOp),
  If(If),
  Cond(Cond),
  Match(Match),
}

pub struct UseTree {
  prefix: Spanned<Path>,
  kind: UseTreeKind,
}

pub enum UseTreeKind {
  Leaf(Spanned<Ident>),
  Branch(Vec<Spanned<UseTree>>),
}

pub struct ModDef {
  name: Ident,
  body: Option<ModBody>,
  public: bool,
}

pub struct ModBody {
  stmts: Vec<Spanned<Stmt>>,
}

pub struct FnDef {
  name: Ident,
  body: Option<FnBody>,
  public: bool,
}

pub struct FnBody {
  block: Block,
  template: Option<Path>,
}

pub enum Lit {
  Unit,
  Int(i64),
  Float(f64),
  String(String),
  Bool(bool),
}

pub struct Label {
  label: Spanned<Path>,
  arg: Box<Spanned<Expr>>,
}

pub struct Assign {
  lhs: Spanned<Pat>,
  rhs: Box<Spanned<Expr>>,
}

pub struct BinOp {
  kind: BinOpKind,
  lhs: Box<Spanned<Expr>>,
  rhs: Box<Spanned<Expr>>,
}

pub enum BinOpKind {
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  And,
  Or,
  Gt,
  Gte,
  Lt,
  Lte,
  Eq,
  Neq,
  Pipe,
}

pub struct UnOp {
  kind: UnOpKind,
  arg: Box<Spanned<Expr>>,
}

pub enum UnOpKind {
  Neg,
  Not,
}

pub struct If {
  head: Box<Spanned<Expr>>,
  body: Block,
  else_: Else,
}

pub enum Else {
  Empty,
  Else(Block),
  If(Box<If>),
}

pub struct Cond {
  arms: Vec<(Box<Spanned<Expr>>, Box<Spanned<Expr>>)>,
}

pub struct Match {
  head: Box<Spanned<Expr>>,
  arms: Vec<(Spanned<Pat>, Box<Spanned<Expr>>)>,
}

pub enum Pat {
  Lit(Lit),
  Tuple(Vec<Spanned<Pat>>),
  Record(Vec<(Spanned<Ident>, Spanned<Pat>)>),
  Label(LabelPat),
}

pub struct LabelPat {
  label: Spanned<Path>,
  arg: Box<Spanned<Pat>>,
}

pub struct Block {
  exprs: Vec<Spanned<Expr>>,
}

pub struct Path {
  idents: Vec<Ident>,
}

pub struct Ident(Intern<String>);

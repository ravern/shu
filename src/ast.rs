use crate::common::{ident::Ident, span::Spanned};

pub enum Stmt {
  Use(UseTree),
  TypeDef(TypeDef),
  ModDef(ModDef),
  FnDef(FnDef),
}

pub enum Expr {
  Lit(Lit),
  Tup(Vec<Spanned<Expr>>),
  Rec(Vec<(Spanned<Ident>, Spanned<Expr>)>),
  Cons(Cons),
  AssOp(Spanned<Pat>, Box<Spanned<Ident>>),
  BinOp(BinOpKind, Box<Spanned<Ident>>, Box<Spanned<Ident>>),
  UnOp(UnOpKind, Box<Spanned<Ident>>),
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

pub struct TypeDef {
  name: Spanned<Ident>,
  public: bool,
}

pub struct ModDef {
  name: Spanned<Ident>,
  body: Option<ModBody>,
  public: bool,
}

pub struct ModBody {
  stmts: Vec<Spanned<Stmt>>,
}

pub struct FnDef {
  name: Spanned<Ident>,
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

pub struct Cons {
  type_: Spanned<Path>,
  arg: Box<Spanned<Expr>>,
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
  Block(Block),
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
  Tup(Vec<Spanned<Pat>>),
  Rec(Vec<(Spanned<Ident>, Spanned<Pat>)>),
  Cons(ConsPat),
}

pub struct ConsPat {
  type_: Spanned<Path>,
  arg: Box<Spanned<Pat>>,
}

pub struct Block {
  exprs: Vec<Spanned<Expr>>,
}

pub struct Path {
  idents: Vec<Ident>,
}

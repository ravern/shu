use internment::Intern;

pub struct Pos {
  from: usize,
  to: usize,
}

pub struct Span {
  line: Pos,
  column: Pos,
}

pub struct Spanned<T> {
  span: Span,
  item: T,
}

impl<T> Spanned<T> {
  pub fn new(span: Span, item: T) -> Spanned<T> {
    Spanned { span, item }
  }
}

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

pub struct ModDef {
  name: Ident,
  body: Option<ModBody>,
}

pub struct ModBody {
  span: Span,
  stmts: Vec<Stmt>,
}

pub struct FnDef {
  name: Ident,
  body: Option<FnBody>,
}

pub struct FnBody {
  exprs: Vec<Expr>,
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
  arg: Spanned<Box<Expr>>,
}

pub struct Assign {
  lhs: Spanned<Pat>,
  rhs: Spanned<Box<Expr>>,
}

pub struct BinOp {
  kind: BinOpKind,
  lhs: Spanned<Box<Expr>>,
  rhs: Spanned<Box<Expr>>,
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
}

pub struct UnOp {
  kind: UnOpKind,
  arg: Spanned<Box<Expr>>,
}

pub enum UnOpKind {
  Neg,
  Not,
}

pub struct If {
  cond: Spanned<Box<Expr>>,
  body: Block,
  else_: Else,
}

pub enum Else {
  Empty,
  Else(Block),
  If(Box<If>),
}

pub enum Pat {
  Lit(Lit),
  Tuple(Vec<Spanned<Pat>>),
  Record(Vec<(Spanned<Ident>, Spanned<Pat>)>),
  Label(LabelPat),
}

pub struct LabelPat {
  label: Spanned<Path>,
  arg: Spanned<Box<Pat>>,
}

pub struct UseTree {}

pub struct Path {
  idents: Vec<Ident>,
}

pub struct Ident(Intern<String>);

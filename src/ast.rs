use crate::common::ident::Ident;

pub struct File(pub Vec<Stmt>);

pub enum Stmt {
  Func {
    name: Ident,
    parameters: Vec<Ident>,
    body: FuncBody,
    public: bool,
  },
  Mod {
    name: Ident,
    body: File,
    public: bool,
  },
}

pub enum FuncBody {
  Impl {
    block: Block,
    template: Option<FuncIdent>,
  },
  Template,
}

pub struct FuncIdent(Vec<Ident>);

pub struct Block(Vec<Expr>);

pub enum Expr {
  Unit,
  Int(i64),
  Float(f64),
  Bool(bool),
  String(String),
  Tuple(Vec<Expr>),
  Record(Vec<(Ident, Expr)>),
  Closure {
    // frame: Frame,
    parameters: Vec<Ident>,
    body: Block,
  },
  Ident(Ident),
  FuncIdent(FuncIdent),
  Label(TypeIdent, Box<Expr>),
  Unary(UnaryOp, Box<Expr>),
  Binary(BinaryOp, Box<Expr>, Box<Expr>),
  Assign(Pattern, Box<Expr>),
  Call(FuncIdent, Vec<Expr>),
  If(If),
  Cond(Vec<(Expr, Block)>),
  Match(Box<Expr>, Vec<(Pattern, Block)>),
  For {
    head: ForHead,
    body: Block,
  },
}

pub struct TypeIdent(Vec<Ident>);

pub enum UnaryOp {
  Negate,
  Not,
}

pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Greater,
  Less,
  Equal,
  GreaterEqual,
  LessEqual,
  And,
  Or,
  Pipe,
}

pub enum Pattern {
  Label(TypeIdent, Box<Pattern>),
}

pub struct If {
  head: Box<Expr>,
  body: Block,
  tail: IfTail,
}

pub enum IfTail {
  Else(Block),
  ElseIf(Box<If>),
  Done,
}

pub enum ForHead {
  Cond(Box<Expr>),
  Iter(Pattern, Box<Expr>),
}

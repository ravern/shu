use internment::Intern;

pub struct File(Vec<Stmt>);

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
  }
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
  For(For),
}

pub struct TypeIdent(Vec<Ident>);

pub enum UnaryOp {
  Negate, Not,
}

pub struct Ident(Intern<String>);
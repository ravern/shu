#[derive(Debug, PartialEq)]
pub enum Expr {
  List(List),
  Atom(Atom),
}

#[derive(Debug, PartialEq)]
pub enum List {
  Cons(Box<Node>),
  Nil,
}

impl List {
  pub fn cons(head: Expr, tail: List) -> List {
    List::Cons(Box::new(Node { head, tail }))
  }
}

#[derive(Debug, PartialEq)]
pub struct Node {
  head: Expr,
  tail: List,
}

#[derive(Debug, PartialEq)]
pub enum Atom {
  String(String),
  Int(i64),
  Float(f64),
  Ident(String),
}

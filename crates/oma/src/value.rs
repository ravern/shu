use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Value {
  List(List),
  Atom(Atom),
}

#[derive(Debug, PartialEq)]
pub enum List {
  Cons(Rc<Node>),
  Nil,
}

impl List {
  pub fn cons(head: Value, tail: List) -> List {
    List::Cons(Rc::new(Node { head, tail }))
  }
}

#[derive(Debug, PartialEq)]
pub struct Node {
  head: Value,
  tail: List,
}

#[derive(Debug, PartialEq)]
pub enum Atom {
  Int(i64),
  Float(f64),
  String(Rc<String>),
  Symbol(Symbol),
  Path(Rc<Vec<Rc<String>>>),
  Lambda(Lambda),
  // Macro(Macro),
  // Module(Module),
  Special(Special),
}

#[derive(Debug)]
pub struct Symbol(Rc<String>);

impl Symbol {
  pub fn new(symbol: String) -> Symbol {
    Symbol(Rc::new(symbol))
  }
}

impl Clone for Symbol {
  fn clone(&self) -> Self {
    Self(Rc::clone(&self.0))
  }
}

impl PartialEq for Symbol {
  fn eq(&self, other: &Self) -> bool {
    Rc::ptr_eq(&self.0, &other.0)
  }
}

#[derive(Debug, PartialEq)]
pub struct Lambda {
  chunk: usize,
}

#[derive(Debug, PartialEq)]
pub enum Special {
  If,
  Lambda,
  Quote,
}

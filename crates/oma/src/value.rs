use std::rc::Rc;

#[derive(Clone)]
pub enum Value {
  List(List),
  Atom(Atom),
}

pub enum List {
  Cons(Rc<Cell>),
  Nil,
}

impl Clone for List {
  fn clone(&self) -> Self {
    match self {
      Self::Cons(cell) => Self::Cons(Rc::clone(cell)),
      Self::Nil => Self::Nil,
    }
  }
}

pub struct Cell {
  head: Value,
  tail: List,
}

#[derive(Clone)]
pub enum Atom {
  Lambda(Lambda),
  Float(f64),
  Integer(i64),
}

#[derive(Clone)]
pub struct Lambda {
  chunk: usize,
}

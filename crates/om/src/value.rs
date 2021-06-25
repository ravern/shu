use std::rc::Rc;

pub enum Value {
  List(List),
  Atom(Atom),
}

pub enum List {
  Cons(Rc<Cell>),
  Nil,
}

pub struct Cell {
  head: Value,
  tail: List,
}

#[derive(Clone)]
pub enum Atom {
  String(String),
}

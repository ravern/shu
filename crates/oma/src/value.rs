use std::rc::Rc;

pub enum Value {
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

pub struct Symbol(Rc<String>);

pub struct Lambda {
  chunk: usize,
}

pub enum Special {
  If,
  Lambda,
  Quote,
}

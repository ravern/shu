use std::{collections::HashMap, rc::Rc};

pub struct Context {
  symbols: HashMap<String, Rc<String>>,
}

impl Context {
  pub fn new() -> Context {
    Context {
      symbols: HashMap::new(),
    }
  }
}

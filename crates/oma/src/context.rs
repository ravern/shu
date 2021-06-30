use std::{collections::HashMap, rc::Rc};

use crate::value::Symbol;

pub struct Context {
  symbols: HashMap<String, Symbol>,
}

impl Context {
  pub fn new() -> Context {
    Context {
      symbols: HashMap::new(),
    }
  }

  pub fn symbol<S>(&mut self, string: S) -> Symbol
  where
    S: Into<String>,
  {
    let string = string.into();
    if let Some(symbol) = self.symbols.get(&string) {
      symbol.clone()
    } else {
      let symbol = Symbol::new(string.clone());
      self.symbols.insert(string, symbol.clone());
      symbol
    }
  }
}

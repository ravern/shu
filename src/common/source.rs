use std::{path::PathBuf, rc::Rc};

#[derive(Debug, PartialEq)]
pub struct Source {
  source: String,
  path: PathBuf,
}

impl Source {
  pub fn new(source: String, path: PathBuf) -> Rc<Source> {
    Rc::new(Source { source, path })
  }
}

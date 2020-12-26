use std::{path::PathBuf, rc::Rc};

#[derive(Debug, PartialEq)]
pub struct Source {
  contents: String,
  path: Option<PathBuf>,
}

impl Source {
  pub fn new(contents: String) -> Rc<Source> {
    Rc::new(Source {
      contents,
      path: None,
    })
  }

  pub fn with_path(contents: String, path: PathBuf) -> Rc<Source> {
    Rc::new(Source {
      contents,
      path: Some(path),
    })
  }

  pub fn contents(&self) -> &str {
    &self.contents
  }

  pub fn len(&self) -> usize {
    self.contents.len()
  }
}

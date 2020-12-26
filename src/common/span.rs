use std::rc::Rc;

use crate::common::source::Source;

#[derive(Debug, PartialEq)]
pub struct Span {
  source: Option<Rc<Source>>,
  offset: usize,
  len: usize,
}

impl Span {
  pub fn new(source: &Rc<Source>, offset: usize, len: usize) -> Span {
    Span {
      source: Some(Rc::clone(source)),
      offset,
      len,
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Spanned<T> {
  span: Span,
  item: T,
}

impl<T> Spanned<T> {
  pub fn new(span: Span, item: T) -> Spanned<T> {
    Spanned { span, item }
  }

  pub fn span(&self) -> &Span {
    &self.span
  }

  pub fn item(&self) -> &T {
    &self.item
  }

  pub fn map<F, U>(self, mut f: F) -> Spanned<U>
  where
    F: FnMut(T) -> U,
  {
    Spanned::new(self.span, f(self.item))
  }
}

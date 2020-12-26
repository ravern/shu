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

  pub fn combine(first: &Span, second: &Span) -> Span {
    if first.source != second.source {
      panic!("can't combine two Spans with separate sources")
    }

    let offset = first.offset.min(second.offset);
    let end = first.end().max(second.end());
    let length = end - offset;

    // TODO: Handle spans without sources
    return Span::new(&first.source.as_ref().unwrap(), offset, length);
  }

  pub fn start(&self) -> usize {
    self.offset
  }

  pub fn end(&self) -> usize {
    self.offset + self.len
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

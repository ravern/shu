use std::rc::Rc;

#[derive(Debug)]
pub struct Spanned<T> {
  base: T,
  span: Span,
}

impl<T> Spanned<T> {
  pub fn new(base: T, span: Span) -> Spanned<T> {
    Spanned { base, span }
  }

  pub fn span(&self) -> &Span {
    &self.span
  }

  pub fn base(&self) -> &T {
    &self.base
  }
}

impl<T> Clone for Spanned<T>
where
  T: Clone,
{
  fn clone(&self) -> Self {
    Self {
      span: self.span.clone(),
      base: self.base.clone(),
    }
  }
}

impl<T> PartialEq for Spanned<T>
where
  T: PartialEq,
{
  fn eq(&self, other: &Self) -> bool {
    self.base == other.base && self.span == other.span
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
  source: Source,
  line: usize,
  column: usize,
  start: usize,
  end: usize,
}

impl Span {
  pub fn new(
    source: Source,
    line: usize,
    column: usize,
    start: usize,
    end: usize,
  ) -> Span {
    if source.len() < end {
      panic!("invalid bounds provided for span");
    }
    Span {
      source,
      line,
      column,
      start,
      end,
    }
  }

  pub fn as_str(&self) -> &str {
    self.source.slice(self.start, self.end)
  }
}

#[derive(Clone, Debug)]
pub struct Source {
  // TODO: Replace with interned string
  inner: Rc<String>,
}

impl Source {
  pub fn new(source: &str) -> Source {
    Source {
      inner: Rc::new(source.to_string()),
    }
  }

  // TODO: Replace with std::ops::Index implementation
  pub fn slice(&self, start: usize, end: usize) -> &str {
    &self.inner.as_str()[start..end]
  }

  // TODO: Replace with std::ops::Index implementation
  pub fn get(&self, index: usize) -> Option<u8> {
    self.inner.as_bytes().get(index).copied()
  }

  pub fn len(&self) -> usize {
    self.inner.len()
  }
}

impl PartialEq for Source {
  fn eq(&self, other: &Self) -> bool {
    Rc::ptr_eq(&self.inner, &other.inner)
  }
}

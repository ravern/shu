#[derive(Debug, PartialEq)]
pub struct Pos {
  line: usize,
  column: usize,
}

impl Pos {
  pub fn new(line: usize, column: usize) -> Pos {
    Pos { line, column }
  }
}

impl From<(usize, usize)> for Pos {
  fn from(pos: (usize, usize)) -> Pos {
    Pos {
      line: pos.0,
      column: pos.1,
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Span {
  from: Pos,
  to: Pos,
}

impl Span {
  pub fn new(from: Pos, to: Pos) -> Span {
    Span { from, to }
  }
}

impl From<((usize, usize), (usize, usize))> for Span {
  fn from(span: ((usize, usize), (usize, usize))) -> Span {
    Span {
      from: span.0.into(),
      to: span.1.into(),
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
}

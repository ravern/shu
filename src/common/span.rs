pub struct Pos {
  from: usize,
  to: usize,
}

impl Pos {
  pub fn new(from: usize, to: usize) -> Pos {
    Pos { from, to }
  }
}

impl From<(usize, usize)> for Pos {
  fn from(pos: (usize, usize)) -> Pos {
    Pos {
      from: pos.0,
      to: pos.1,
    }
  }
}

pub struct Span {
  lines: Pos,
  columns: Pos,
}

impl Span {
  pub fn new(lines: Pos, columns: Pos) -> Span {
    Span { lines, columns }
  }
}

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

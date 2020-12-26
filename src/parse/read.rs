use std::rc::Rc;

use crate::{common::source::Source, parse::ParseError};

pub struct Reader {
  source: Rc<Source>,
  offset: usize,
}

impl Reader {
  pub fn new(source: &Rc<Source>) -> Reader {
    Reader {
      source: Rc::clone(source),
      offset: 0,
    }
  }

  pub fn is_empty(&self) -> bool {
    self.offset == self.source.len()
  }

  pub fn peek(&self) -> Option<(u8, Option<u8>)> {
    let contents = self.source.contents().as_bytes();

    let first = contents.get(self.offset).cloned();
    let second = contents.get(self.offset + 1).cloned();

    match first {
      Some(first) => match second {
        Some(second) => Some((first, Some(second))),
        None => Some((first, None)),
      },
      None => None,
    }
  }

  pub fn advance(&mut self) -> Option<(u8, Option<u8>)> {
    let bytes = self.peek();

    match bytes {
      Some((first, Some(second))) => {
        self.offset += 2;
        bytes
      }
      Some((first, None)) => {
        self.offset += 1;
        bytes
      }
      None => None,
    }
  }

  pub fn expect(
    &mut self,
    chars: (u8, Option<u8>),
  ) -> Option<(u8, Option<u8>)> {
    if self.advance() == Some(chars) {
      Some(chars)
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::common::source::Source;

  use super::Reader;

  #[test]
  fn is_empty() {
    let mut reader = Reader::new(&Source::with_contents("abc".to_string()));
    reader.advance();
    assert_eq!(reader.is_empty(), false);
    reader.advance();
    assert_eq!(reader.is_empty(), true);
  }

  #[test]
  fn peek() {
    let reader = Reader::new(&Source::with_contents("abc".to_string()));
    assert_eq!(reader.peek(), Some((b'a', Some(b'b'))));
    assert_eq!(reader.peek(), Some((b'a', Some(b'b'))));
  }

  #[test]
  fn advance() {
    let mut reader = Reader::new(&Source::with_contents("abc".to_string()));
    assert_eq!(reader.advance(), Some((b'a', Some(b'b'))));
    assert_eq!(reader.advance(), Some((b'c', None)));
  }

  #[test]
  fn expect() {
    let mut reader = Reader::new(&Source::with_contents("abc".to_string()));
    assert_eq!(reader.expect((b'a', Some(b'b'))), Some((b'a', Some(b'b'))));
    assert_eq!(reader.expect((b'c', None)), Some((b'c', None)));
  }
}

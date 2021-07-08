#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
  Int,
  Float,
  Plus,
  Hyphen,
  Asterisk,
  Slash,
  Comment,
  Newline,
  Eof,
}

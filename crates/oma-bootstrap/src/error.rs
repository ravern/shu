#[derive(Debug, PartialEq)]
pub enum Error {
  UnexpectedEof,
  UnexpectedChar(char),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
  Int,
  Float,
  String,
  Plus,
  Dash,
  Star,
  Slash,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  Equal,
  EqualEqual,
  Bang,
  BangEqual,
  AmpAmp,
  PipePipe,
  Comma,
  Period,
  Semicolon,
  OpenParen,
  CloseParen,
  OpenBrace,
  CloseBrace,
  OpenBracket,
  CloseBracket,
  Identifier,
  Mod,
  Fn,
  Impl,
  Let,
  Mut,
  True,
  False,
  Comment,
  Eof,
}

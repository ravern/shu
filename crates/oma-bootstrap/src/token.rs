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
  ColonColon,
  Semicolon,
  OpenParen,
  CloseParen,
  OpenBrace,
  CloseBrace,
  OpenBracket,
  CloseBracket,
  Identifier,
  Use,
  Mod,
  Fn,
  Impl,
  Let,
  Mut,
  If,
  Else,
  While,
  True,
  False,
  Comment,
  Eof,
}

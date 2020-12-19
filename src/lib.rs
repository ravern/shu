mod ast;
mod common;
mod lex;
mod parse;
mod token;

pub fn run() {
  let source = fallible_iterator::convert::<_, std::io::Error, _>("use test".chars().map(Ok));
  let tokens = lex::lex(source).unwrap();

  let tokens =
    fallible_iterator::convert::<_, lex::LexError<std::io::Error>, _>(tokens.into_iter().map(Ok));
  let file = parse::parse(tokens).unwrap();
}

use std::iter::Peekable;

use thiserror::Error;

use crate::{context::Context, value::*};

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
  #[error("unexpected end of file")]
  UnexpectedEof,
  #[error("unexpected char: {0}")]
  UnexpectedChar(char),
}

pub fn parse(
  context: &mut Context,
  source: &str,
) -> Result<Vec<Value>, ParseError> {
  let mut parser = Parser::new(context, source.chars());

  let mut values = Vec::new();
  loop {
    match parser.try_value()? {
      Some(value) => values.push(value),
      None => break,
    }
  }

  Ok(values)
}

struct Parser<'a, S>
where
  S: Iterator<Item = char>,
{
  context: &'a mut Context,
  source: Peekable<S>,
}

impl<'a, S> Parser<'a, S>
where
  S: Iterator<Item = char>,
{
  fn new(context: &'a mut Context, source: S) -> Parser<'a, S> {
    Parser {
      context,
      source: source.peekable(),
    }
  }

  fn try_value(&mut self) -> Result<Option<Value>, ParseError> {
    match self.try_peek()? {
      Some(_) => Ok(Some(self.value()?)),
      None => Ok(None),
    }
  }

  fn value(&mut self) -> Result<Value, ParseError> {
    let value = match self.peek()? {
      '(' => Value::List(self.list()?),
      _ => Value::Atom(self.atom()?),
    };
    self.whitespace();
    Ok(value)
  }

  fn list(&mut self) -> Result<List, ParseError> {
    self.expect('(')?;

    let mut values = Vec::new();
    loop {
      match self.peek()? {
        ')' => break,
        _ => values.push(self.value()?),
      }
    }

    let mut list = List::Nil;
    while let Some(value) = values.pop() {
      list = List::cons(value, list);
    }

    self.expect(')')?;

    Ok(list)
  }

  fn atom(&mut self) -> Result<Atom, ParseError> {
    match self.peek()? {
      char if char.is_digit(10) => self.number(),
      _ => self.symbol().map(Atom::Symbol),
    }
  }

  fn number(&mut self) -> Result<Atom, ParseError> {
    let mut chars = Vec::new();
    let mut has_decimal = false;

    loop {
      match self.try_peek()? {
        Some('.') if !has_decimal => has_decimal = true,
        Some(char) if char.is_digit(10) => {}
        Some(')') => break,
        Some(char) if char.is_whitespace() => break,
        Some(char) => return Err(ParseError::UnexpectedChar(char)),
        None => break,
      }
      chars.push(self.next()?)
    }

    let string = chars.into_iter().collect::<String>();

    if has_decimal {
      Ok(Atom::Float(
        string
          .parse()
          .expect("non-numerical char allowed into chars"),
      ))
    } else {
      Ok(Atom::Int(
        string
          .parse()
          .expect("non-numerical char allowed into chars"),
      ))
    }
  }

  fn symbol(&mut self) -> Result<Symbol, ParseError> {
    let mut chars = Vec::new();

    loop {
      match self.try_peek()? {
        Some(')') => break,
        Some(char) if char.is_whitespace() => break,
        Some(char) => {}
        None => break,
      }
      chars.push(self.next()?);
    }

    let string: String = chars.into_iter().collect();
    Ok(self.context.symbol(string))
  }

  fn whitespace(&mut self) {
    loop {
      match self.source.peek() {
        Some(char) if char.is_whitespace() => self.source.next(),
        _ => break,
      };
    }
  }

  fn expect(&mut self, char: char) -> Result<(), ParseError> {
    if self.next()? == char {
      self.whitespace();
      Ok(())
    } else {
      Err(ParseError::UnexpectedChar(char))
    }
  }

  fn try_next(&mut self) -> Result<Option<char>, ParseError> {
    Ok(self.source.next())
  }

  fn next(&mut self) -> Result<char, ParseError> {
    self.try_next()?.ok_or(ParseError::UnexpectedEof)
  }

  fn try_peek(&mut self) -> Result<Option<char>, ParseError> {
    Ok(self.source.peek().cloned())
  }

  fn peek(&mut self) -> Result<char, ParseError> {
    self.try_peek()?.ok_or(ParseError::UnexpectedEof)
  }
}

#[cfg(test)]
mod tests {
  use crate::{compile::parse::parse, context::Context, value::*};

  #[test]
  fn int() {
    let mut context = Context::new();
    assert_eq!(
      parse(&mut context, "5"),
      Ok(vec![Value::Atom(Atom::Int(5))])
    );
  }

  #[test]
  fn float() {
    let mut context = Context::new();
    assert_eq!(
      parse(&mut context, "30.56"),
      Ok(vec![Value::Atom(Atom::Float(30.56))])
    );
  }

  #[test]
  fn symbol() {
    let mut context = Context::new();
    assert_eq!(
      parse(&mut context, "thirty-two"),
      Ok(vec![Value::Atom(Atom::Symbol(
        context.symbol("thirty-two".to_string())
      ))])
    );
  }

  #[test]
  fn list() {
    let mut context = Context::new();
    assert_eq!(
      parse(&mut context, "(1 2 3)"),
      Ok(vec![Value::List(List::cons(
        Value::Atom(Atom::Int(1)),
        List::cons(
          Value::Atom(Atom::Int(2)),
          List::cons(Value::Atom(Atom::Int(3)), List::Nil)
        )
      ))])
    );
  }

  #[test]
  fn complex() {
    let mut context = Context::new();
    assert_eq!(
      parse(
        &mut context,
        "(fn add (lambda (left right) (+ left right)))"
      ),
      Ok(vec![Value::List(List::cons(
        Value::Atom(Atom::Symbol(context.symbol("fn"))),
        List::cons(
          Value::Atom(Atom::Symbol(context.symbol("add"))),
          List::cons(
            Value::List(List::cons(
              Value::Atom(Atom::Symbol(context.symbol("lambda"))),
              List::cons(
                Value::List(List::cons(
                  Value::Atom(Atom::Symbol(context.symbol("left"))),
                  List::cons(
                    Value::Atom(Atom::Symbol(context.symbol("right"))),
                    List::Nil
                  )
                )),
                List::cons(
                  Value::List(List::cons(
                    Value::Atom(Atom::Symbol(context.symbol("+"))),
                    List::cons(
                      Value::Atom(Atom::Symbol(context.symbol("left"))),
                      List::cons(
                        Value::Atom(Atom::Symbol(context.symbol("right"))),
                        List::Nil
                      ),
                    ),
                  )),
                  List::Nil
                )
              ),
            )),
            List::Nil
          )
        )
      ))])
    );
  }
}

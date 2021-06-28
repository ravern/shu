use std::iter::Peekable;

use crate::{ast::*, error::Error};

pub fn parse(source: &str) -> Result<Vec<Expr>, Error> {
  let mut parser = Parser::new(source.chars());
  let mut exprs = Vec::new();
  loop {
    match parser.try_expr()? {
      Some(expr) => exprs.push(expr),
      None => break,
    }
  }
  Ok(exprs)
}

struct Parser<S>
where
  S: Iterator<Item = char>,
{
  source: Peekable<S>,
}

impl<S> Parser<S>
where
  S: Iterator<Item = char>,
{
  fn new(source: S) -> Parser<S> {
    Parser {
      source: source.peekable(),
    }
  }

  fn try_expr(&mut self) -> Result<Option<Expr>, Error> {
    match self.try_peek()? {
      Some(_) => Ok(Some(self.expr()?)),
      None => Ok(None),
    }
  }

  fn expr(&mut self) -> Result<Expr, Error> {
    let expr = match self.peek()? {
      '(' => Expr::List(self.list()?),
      _ => Expr::Atom(self.atom()?),
    };
    self.whitespace();
    Ok(expr)
  }

  fn list(&mut self) -> Result<List, Error> {
    self.expect('(')?;

    let mut exprs = Vec::new();
    loop {
      match self.peek()? {
        ')' => break,
        _ => exprs.push(self.expr()?),
      }
    }

    let mut list = List::Nil;
    while let Some(expr) = exprs.pop() {
      list = List::cons(expr, list);
    }

    self.expect(')')?;

    Ok(list)
  }

  fn atom(&mut self) -> Result<Atom, Error> {
    match self.peek()? {
      char if char.is_digit(10) => self.number(),
      _ => self.ident().map(Atom::Ident),
    }
  }

  fn number(&mut self) -> Result<Atom, Error> {
    let mut chars = Vec::new();
    let mut has_decimal = false;

    loop {
      match self.try_peek()? {
        Some('.') if !has_decimal => has_decimal = true,
        Some(char) if char.is_digit(10) => {}
        Some(')') => break,
        Some(char) if char.is_whitespace() => break,
        Some(char) => return Err(Error::UnexpectedChar(char)),
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

  fn ident(&mut self) -> Result<String, Error> {
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

    Ok(chars.into_iter().collect())
  }

  fn whitespace(&mut self) {
    loop {
      match self.source.peek() {
        Some(char) if char.is_whitespace() => self.source.next(),
        _ => break,
      };
    }
  }

  fn expect(&mut self, char: char) -> Result<(), Error> {
    if self.next()? == char {
      self.whitespace();
      Ok(())
    } else {
      Err(Error::UnexpectedChar(char))
    }
  }

  fn try_next(&mut self) -> Result<Option<char>, Error> {
    Ok(self.source.next())
  }

  fn next(&mut self) -> Result<char, Error> {
    self.try_next()?.ok_or(Error::UnexpectedEof)
  }

  fn try_peek(&mut self) -> Result<Option<char>, Error> {
    Ok(self.source.peek().cloned())
  }

  fn peek(&mut self) -> Result<char, Error> {
    self.try_peek()?.ok_or(Error::UnexpectedEof)
  }
}

#[cfg(test)]
mod tests {
  use crate::{ast::*, parse::parse};

  #[test]
  fn int() {
    assert_eq!(parse("5"), Ok(vec![Expr::Atom(Atom::Int(5))]));
  }

  #[test]
  fn float() {
    assert_eq!(parse("30.56"), Ok(vec![Expr::Atom(Atom::Float(30.56))]));
  }

  #[test]
  fn ident() {
    assert_eq!(
      parse("thirty-two"),
      Ok(vec![Expr::Atom(Atom::Ident("thirty-two".to_string()))])
    );
  }

  #[test]
  fn list() {
    assert_eq!(
      parse("(1 2 3)"),
      Ok(vec![Expr::List(List::cons(
        Expr::Atom(Atom::Int(1)),
        List::cons(
          Expr::Atom(Atom::Int(2)),
          List::cons(Expr::Atom(Atom::Int(3)), List::Nil)
        )
      ))])
    );
  }

  #[test]
  fn complex() {
    assert_eq!(
      parse("(def add (lambda (left right) (+ left right)))"),
      Ok(vec![Expr::List(List::cons(
        Expr::Atom(Atom::Ident("def".to_string())),
        List::cons(
          Expr::Atom(Atom::Ident("add".to_string())),
          List::cons(
            Expr::List(List::cons(
              Expr::Atom(Atom::Ident("lambda".to_string())),
              List::cons(
                Expr::List(List::cons(
                  Expr::Atom(Atom::Ident("left".to_string())),
                  List::cons(
                    Expr::Atom(Atom::Ident("right".to_string())),
                    List::Nil
                  )
                )),
                List::cons(
                  Expr::List(List::cons(
                    Expr::Atom(Atom::Ident("+".to_string())),
                    List::cons(
                      Expr::Atom(Atom::Ident("left".to_string())),
                      List::cons(
                        Expr::Atom(Atom::Ident("right".to_string())),
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

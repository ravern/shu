use crate::{
  ast::{BinaryExpression, Expression, LiteralExpression, UnaryExpression},
  lex::{LexError, Lexer},
  span::{Span, Spanned},
  token::Token,
};

#[derive(Debug, PartialEq)]
pub enum ParseError {
  Lex(LexError),
  UnexpectedToken(Token),
}

pub struct Parser {
  lexer: Lexer,
  current: Option<Spanned<Token>>,
}

impl Parser {
  pub fn new(source: &str) -> Parser {
    Parser {
      lexer: Lexer::new(source),
      current: None,
    }
  }

  pub fn parse(&mut self) -> Result<Spanned<Expression>, Spanned<ParseError>> {
    self.expression()
  }

  fn expression(&mut self) -> Result<Spanned<Expression>, Spanned<ParseError>> {
    self.multiplication()
  }

  fn multiplication(
    &mut self,
  ) -> Result<Spanned<Expression>, Spanned<ParseError>> {
    self.addition()
  }

  fn addition(&mut self) -> Result<Spanned<Expression>, Spanned<ParseError>> {
    let left_operand = Box::new(self.unary()?);

    let operator = match self.peek()?.base() {
      Token::Plus | Token::Hyphen => self.advance()?,
      _ => {
        let token = self.advance()?;
        return Err(token.map(ParseError::UnexpectedToken));
      }
    };

    let right_operand = Box::new(self.unary()?);

    let span = Span::combine(
      left_operand.span(),
      &Span::combine(operator.span(), right_operand.span()),
    );
    Ok(Spanned::new(
      Expression::Binary(BinaryExpression {
        operator,
        left_operand,
        right_operand,
      }),
      span,
    ))
  }

  fn unary(&mut self) -> Result<Spanned<Expression>, Spanned<ParseError>> {
    match self.peek()?.base() {
      Token::Hyphen => {
        let operator = self.advance()?;
        let operand = Box::new(self.unary()?);
        let span = Span::combine(operator.span(), operand.span());
        let expression = Spanned::new(
          Expression::Unary(UnaryExpression { operator, operand }),
          span,
        );
        Ok(expression)
      }
      _ => self.literal(),
    }
  }

  fn literal(&mut self) -> Result<Spanned<Expression>, Spanned<ParseError>> {
    match self.peek()?.base() {
      Token::Int => {
        let int = self.advance()?;
        let expression = Spanned::new(
          Expression::Literal(LiteralExpression::Int(
            int.span().as_str().parse().unwrap(),
          )),
          int.span().clone(),
        );
        Ok(expression)
      }
      Token::Float => {
        let int = self.advance()?;
        let expression = Spanned::new(
          Expression::Literal(LiteralExpression::Int(
            int.span().as_str().parse().unwrap(),
          )),
          int.span().clone(),
        );
        Ok(expression)
      }
      _ => {
        let token = self.advance()?;
        Err(token.map(ParseError::UnexpectedToken))
      }
    }
  }

  fn peek(&mut self) -> Result<Spanned<Token>, Spanned<ParseError>> {
    if let Some(token) = &self.current {
      return Ok(token.clone());
    }

    let mut error = None;

    loop {
      match self.lexer.next() {
        Ok(token) => {
          self.current = Some(token);
          break;
        }
        Err(lex_error) => {
          if error.is_none() {
            error = Some(lex_error.map(ParseError::Lex));
          }
        }
      }
    }

    if let Some(error) = error {
      Err(error)
    } else {
      Ok(self.current.as_ref().cloned().unwrap())
    }
  }

  fn advance(&mut self) -> Result<Spanned<Token>, Spanned<ParseError>> {
    self.peek()?;
    Ok(self.current.take().unwrap())
  }

  fn expect(
    &mut self,
    token: Token,
  ) -> Result<Spanned<Token>, Spanned<ParseError>> {
    let spanned_token = self.advance()?;

    if spanned_token.base() == &token {
      Ok(spanned_token)
    } else {
      Err(spanned_token.map(ParseError::UnexpectedToken))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Parser;

  #[test]
  fn addition() {
    let mut parser = Parser::new("1 + 1");
    dbg!(parser.parse());
  }
}

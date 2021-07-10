use crate::{
  ast::Expression,
  lex::{LexError, Lexer},
  span::{Source, Span, Spanned},
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

macro_rules! binary {
  ($self:ident, $parse:ident, $($tokens:pat)|+ $(,)?) => {
    let mut left_operand = $self.$parse()?;

    loop {
      let operator = match $self.peek()?.base() {
        $($tokens)|+ => $self.advance()?,
        _ => return Ok(left_operand),
      };

      let right_operand = $self.$parse()?;

      let span = Span::combine(
        left_operand.span(),
        &Span::combine(operator.span(), right_operand.span()),
      );
      left_operand = Spanned::new(
        Expression::binary(operator, left_operand, right_operand),
        span,
      );
    }
  };
}

impl Parser {
  pub fn new(source: &str) -> Parser {
    Parser {
      lexer: Lexer::new(source),
      current: None,
    }
  }

  pub fn source(&self) -> &Source {
    self.lexer.source()
  }

  pub fn parse(mut self) -> Spanned<Expression> {
    let expression = self.expression().unwrap();
    self.expect(Token::Eof).unwrap();
    expression
  }

  fn expression(&mut self) -> Result<Spanned<Expression>, Spanned<ParseError>> {
    self.logical()
  }

  fn logical(&mut self) -> Result<Spanned<Expression>, Spanned<ParseError>> {
    binary! {
      self,
      equality,
      Token::AmpAmp | Token::PipePipe,
    }
  }

  fn equality(&mut self) -> Result<Spanned<Expression>, Spanned<ParseError>> {
    let left_operand = self.comparison()?;

    let operator = match self.peek()?.base() {
      Token::EqualEqual | Token::BangEqual => self.advance()?,
      _ => return Ok(left_operand),
    };

    let right_operand = self.comparison()?;

    let span = Span::combine(
      left_operand.span(),
      &Span::combine(operator.span(), right_operand.span()),
    );
    let expression = Spanned::new(
      Expression::binary(operator, left_operand, right_operand),
      span,
    );

    let token = self.peek()?;
    match token.base() {
      Token::EqualEqual | Token::BangEqual => {
        let span = token.span().clone();
        return Err(Spanned::new(
          ParseError::UnexpectedToken(token.unwrap()),
          span,
        ));
      }
      _ => {}
    }

    Ok(expression)
  }

  fn comparison(&mut self) -> Result<Spanned<Expression>, Spanned<ParseError>> {
    let left_operand = self.addition()?;

    let operator = match self.peek()?.base() {
      Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual => {
        self.advance()?
      }
      _ => return Ok(left_operand),
    };

    let right_operand = self.addition()?;

    let span = Span::combine(
      left_operand.span(),
      &Span::combine(operator.span(), right_operand.span()),
    );
    let expression = Spanned::new(
      Expression::binary(operator, left_operand, right_operand),
      span,
    );

    let token = self.peek()?;
    match token.base() {
      Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual => {
        let span = token.span().clone();
        return Err(Spanned::new(
          ParseError::UnexpectedToken(token.unwrap()),
          span,
        ));
      }
      _ => {}
    }

    Ok(expression)
  }

  fn addition(&mut self) -> Result<Spanned<Expression>, Spanned<ParseError>> {
    binary! {
      self,
      multiplication,
      Token::Plus | Token::Dash,
    }
  }

  fn multiplication(
    &mut self,
  ) -> Result<Spanned<Expression>, Spanned<ParseError>> {
    binary! {
      self,
      unary,
      Token::Star | Token::Slash,
    }
  }

  fn unary(&mut self) -> Result<Spanned<Expression>, Spanned<ParseError>> {
    match self.peek()?.base() {
      Token::Dash | Token::Bang => {
        let operator = self.advance()?;
        let operand = self.unary()?;
        let span = Span::combine(operator.span(), operand.span());
        Ok(Spanned::new(Expression::unary(operator, operand), span))
      }
      Token::OpenParen => {
        let open_paren = self.advance()?;
        let expression = self.expression()?;
        let close_paren = self.expect(Token::CloseParen)?;
        let span = Span::combine(
          open_paren.span(),
          &Span::combine(expression.span(), close_paren.span()),
        );
        Ok(Spanned::new(expression.unwrap(), span))
      }
      _ => self.literal(),
    }
  }

  fn literal(&mut self) -> Result<Spanned<Expression>, Spanned<ParseError>> {
    match self.peek()?.base() {
      Token::Int => {
        let int = self.advance()?;
        let expression = Spanned::new(
          Expression::int(int.span().as_str().parse().unwrap()),
          int.span().clone(),
        );
        Ok(expression)
      }
      Token::Float => {
        let float = self.advance()?;
        let expression = Spanned::new(
          Expression::float(float.span().as_str().parse().unwrap()),
          float.span().clone(),
        );
        Ok(expression)
      }
      Token::True => {
        let bool = self.advance()?;
        let expression =
          Spanned::new(Expression::bool(true), bool.span().clone());
        Ok(expression)
      }
      Token::False => {
        let bool = self.advance()?;
        let expression =
          Spanned::new(Expression::bool(false), bool.span().clone());
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
  use pretty_assertions::assert_eq;

  use crate::{
    ast::Expression,
    span::{Span, Spanned},
    token::Token,
  };

  use super::Parser;

  #[test]
  fn operations() {
    let parser = Parser::new("1 + 2 * 3 - 4 / 5");
    let source = parser.source().clone();
    let expression = parser.parse();

    assert_eq!(
      expression,
      Spanned::new(
        Expression::binary(
          Spanned::new(Token::Dash, Span::new(source.clone(), 10, 11)),
          Spanned::new(
            Expression::binary(
              Spanned::new(Token::Plus, Span::new(source.clone(), 2, 3)),
              Spanned::new(Expression::int(1), Span::new(source.clone(), 0, 1)),
              Spanned::new(
                Expression::binary(
                  Spanned::new(Token::Star, Span::new(source.clone(), 6, 7)),
                  Spanned::new(
                    Expression::int(2),
                    Span::new(source.clone(), 4, 5)
                  ),
                  Spanned::new(
                    Expression::int(3),
                    Span::new(source.clone(), 8, 9)
                  ),
                ),
                Span::new(source.clone(), 4, 9)
              ),
            ),
            Span::new(source.clone(), 0, 9),
          ),
          Spanned::new(
            Expression::binary(
              Spanned::new(Token::Slash, Span::new(source.clone(), 14, 15)),
              Spanned::new(
                Expression::int(4),
                Span::new(source.clone(), 12, 13)
              ),
              Spanned::new(
                Expression::int(5),
                Span::new(source.clone(), 16, 17)
              ),
            ),
            Span::new(source.clone(), 12, 17)
          )
        ),
        Span::new(source.clone(), 0, 17)
      ),
    );
  }
}

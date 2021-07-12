use crate::{
  ast::{
    BinaryExpression, BindStatement, Block, ElseBody, Expression,
    ExpressionStatement, File, FnDeclaration, IfExpression, ModDeclaration,
    ParseError, Pattern, Statement, UnaryExpression, UseDeclaration, UseTree,
    UseTreeBranch, WhileExpression,
  },
  lex::Lexer,
  span::{Source, Spanned},
  token::Token,
};

pub struct Parser {
  lexer: Lexer,
  current: Option<Spanned<Token>>,
}

macro_rules! binary {
  ($self:ident, $parse:ident, $($tokens:pat)|+ $(,)?) => {
    let mut left_operand = Box::new($self.$parse()?);

    loop {
      let operator = match $self.peek()?.base() {
        $($tokens)|+ => $self.advance()?,
        _ => return Ok(*left_operand),
      };

      let right_operand = Box::new($self.$parse()?);

      left_operand = Box::new(Expression::Binary(BinaryExpression { left_operand, operator, right_operand }));
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

  pub fn parse(mut self) -> Result<File, Vec<Spanned<ParseError>>> {
    self.file(false)
  }

  fn file(
    &mut self,
    is_within_mod_declaration: bool,
  ) -> Result<File, Vec<Spanned<ParseError>>> {
    let mut file = File::new();
    let mut errors = Vec::new();

    loop {
      let token = match self.peek() {
        Ok(token) => token,
        Err(error) => {
          errors.push(error);
          self.synchronize();
          continue;
        }
      };
      match token.base() {
        Token::Eof => break,
        Token::CloseBrace if is_within_mod_declaration => break,
        Token::Use => match self.use_declaration() {
          Ok(use_declaration) => file.use_declarations.push(use_declaration),
          Err(error) => {
            errors.push(error);
            self.synchronize();
          }
        },
        Token::Fn => match self.fn_declaration() {
          Ok(fn_declaration) => file.fn_declarations.push(fn_declaration),
          Err(error) => {
            errors.push(error);
            self.synchronize();
          }
        },
        Token::Mod => match self.mod_declaration() {
          Ok(mod_declaration) => file.mod_declarations.push(mod_declaration),
          Err(mod_errors) => {
            errors.extend(mod_errors);
            self.synchronize();
          }
        },
        _ => {
          let error = ParseError::UnexpectedToken(token.base().clone());
          errors.push(token.map(|_| error));
          self.synchronize();
        }
      }
    }

    if !is_within_mod_declaration {
      if let Err(error) = self.expect(Token::Eof) {
        errors.push(error);
      }
    }

    if errors.is_empty() {
      Ok(file)
    } else {
      Err(errors)
    }
  }

  fn use_declaration(&mut self) -> Result<UseDeclaration, Spanned<ParseError>> {
    self.expect(Token::Use)?;

    let trees = match self.peek()?.base() {
      Token::Identifier => vec![self.use_tree()?],
      Token::OpenBrace => self.use_trees()?,
      token => {
        return Err(
          self
            .advance()?
            .map(|_| ParseError::UnexpectedToken(token.clone())),
        )
      }
    };

    self.expect(Token::Semicolon)?;

    Ok(UseDeclaration { trees })
  }

  fn use_trees(&mut self) -> Result<Vec<UseTree>, Spanned<ParseError>> {
    self.expect(Token::OpenBrace)?;

    let mut trees = Vec::new();
    loop {
      trees.push(self.use_tree()?);

      if let Token::Comma = self.peek()?.base() {
        self.advance()?;
      }

      if let Token::CloseBrace = self.peek()?.base() {
        break;
      }
    }

    self.expect(Token::CloseBrace)?;

    Ok(trees)
  }

  fn use_tree(&mut self) -> Result<UseTree, Spanned<ParseError>> {
    let component = self.expect(Token::Identifier)?;

    if let Token::ColonColon = self.peek()?.base() {
      self.advance()?;
    } else {
      return Ok(UseTree::Leaf(component));
    }

    let subtrees = match self.peek()?.base() {
      Token::Identifier => vec![self.use_tree()?],
      Token::OpenBrace => self.use_trees()?,
      token => {
        return Err(
          self
            .advance()?
            .map(|_| ParseError::UnexpectedToken(token.clone())),
        )
      }
    };

    Ok(UseTree::Branch(UseTreeBranch {
      component,
      subtrees,
    }))
  }

  // TODO: Improve all the `map_err` calls.
  fn mod_declaration(
    &mut self,
  ) -> Result<ModDeclaration, Vec<Spanned<ParseError>>> {
    self.expect(Token::Mod).map_err(|error| vec![error])?;

    let name = self
      .expect(Token::Identifier)
      .map_err(|error| vec![error])?;

    let body = match self.peek().map_err(|error| vec![error])?.base() {
      Token::OpenBrace => {
        self.advance().map_err(|error| vec![error])?;
        let file = self.file(true)?;
        self
          .expect(Token::CloseBrace)
          .map_err(|error| vec![error])?;
        Some(file)
      }
      Token::Semicolon => {
        self.advance().map_err(|error| vec![error])?;
        None
      }
      token => {
        return Err(vec![self
          .advance()
          .map_err(|error| vec![error])?
          .map(|_| ParseError::UnexpectedToken(token.clone()))]);
      }
    };

    Ok(ModDeclaration { name, body })
  }

  fn fn_declaration(&mut self) -> Result<FnDeclaration, Spanned<ParseError>> {
    self.expect(Token::Fn)?;

    let name = self.expect(Token::Identifier)?;

    self.expect(Token::OpenParen)?;
    let mut parameters = Vec::new();
    loop {
      if let Token::CloseParen = self.peek()?.base() {
        break;
      }

      let parameter = self.expect(Token::Identifier)?;
      parameters.push(parameter);

      if let Token::Comma = self.peek()?.base() {
        self.advance()?;
      }
    }
    self.expect(Token::CloseParen)?;

    let body = self.block()?;

    Ok(FnDeclaration {
      name,
      parameters,
      body,
    })
  }

  fn block(&mut self) -> Result<Block, Spanned<ParseError>> {
    self.expect(Token::OpenBrace)?;

    let mut statements = Vec::new();
    loop {
      if let Token::CloseBrace = self.peek()?.base() {
        break;
      }

      let statement = self.statement()?;
      statements.push(statement);
    }

    self.expect(Token::CloseBrace)?;

    Ok(Block { statements })
  }

  fn statement(&mut self) -> Result<Statement, Spanned<ParseError>> {
    match self.peek()?.base() {
      Token::Let => self.bind_statement(),
      _ => self.expression_statement(),
    }
  }

  fn bind_statement(&mut self) -> Result<Statement, Spanned<ParseError>> {
    self.expect(Token::Let)?;

    let is_mut = match self.peek()?.base() {
      Token::Mut => {
        self.advance()?;
        true
      }
      _ => false,
    };

    let pattern = self.pattern()?.unwrap();

    self.expect(Token::Equal)?;

    let expression = Box::new(self.expression()?);

    self.expect(Token::Semicolon)?;

    Ok(Statement::Bind(BindStatement {
      is_mut,
      pattern,
      expression,
    }))
  }

  fn expression_statement(&mut self) -> Result<Statement, Spanned<ParseError>> {
    let expression = self.expression()?;

    let mut has_semicolon = false;
    if let Token::Semicolon = self.peek()?.base() {
      self.advance()?;
      has_semicolon = true;
    }

    Ok(Statement::Expression(ExpressionStatement {
      expression,
      has_semicolon,
    }))
  }

  fn expression(&mut self) -> Result<Expression, Spanned<ParseError>> {
    let expression = match self.peek()?.base() {
      Token::If => Expression::If(self.if_expression()?),
      Token::While => Expression::While(self.while_expression()?),
      _ => self.logical_expression()?,
    };
    Ok(expression)
  }

  fn if_expression(&mut self) -> Result<IfExpression, Spanned<ParseError>> {
    self.expect(Token::If)?;

    let condition = self.expression()?;

    let body = self.block()?;

    let else_body = self.else_body()?;

    Ok(IfExpression {
      condition: Box::new(condition),
      body,
      else_body,
    })
  }

  fn else_body(&mut self) -> Result<Option<ElseBody>, Spanned<ParseError>> {
    match self.peek()?.base() {
      Token::Else => self.advance()?,
      _ => return Ok(None),
    };

    let block = if let Token::If = self.peek()?.base() {
      ElseBody::If(Box::new(self.if_expression()?))
    } else {
      ElseBody::Else(self.block()?)
    };

    Ok(Some(block))
  }

  fn while_expression(
    &mut self,
  ) -> Result<WhileExpression, Spanned<ParseError>> {
    self.expect(Token::While)?;

    let condition = self.expression()?;
    let body = self.block()?;

    Ok(WhileExpression {
      condition: Box::new(condition),
      body,
    })
  }

  fn logical_expression(&mut self) -> Result<Expression, Spanned<ParseError>> {
    binary! {
      self,
      equality_expression,
      Token::AmpAmp | Token::PipePipe,
    }
  }

  fn equality_expression(&mut self) -> Result<Expression, Spanned<ParseError>> {
    let left_operand = Box::new(self.comparison_expression()?);

    let operator = match self.peek()?.base() {
      Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual => {
        self.advance()?
      }
      _ => return Ok(*left_operand),
    };

    let right_operand = Box::new(self.comparison_expression()?);

    let expression = Expression::Binary(BinaryExpression {
      left_operand,
      operator,
      right_operand,
    });

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

  fn comparison_expression(
    &mut self,
  ) -> Result<Expression, Spanned<ParseError>> {
    let left_operand = Box::new(self.addition_expression()?);

    let operator = match self.peek()?.base() {
      Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual => {
        self.advance()?
      }
      _ => return Ok(*left_operand),
    };

    let right_operand = Box::new(self.addition_expression()?);

    let expression = Expression::Binary(BinaryExpression {
      left_operand,
      operator,
      right_operand,
    });

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

  fn addition_expression(&mut self) -> Result<Expression, Spanned<ParseError>> {
    binary! {
      self,
      multiplication_expression,
      Token::Plus | Token::Dash,
    }
  }

  fn multiplication_expression(
    &mut self,
  ) -> Result<Expression, Spanned<ParseError>> {
    binary! {
      self,
      unary_expression,
      Token::Star | Token::Slash,
    }
  }

  fn unary_expression(&mut self) -> Result<Expression, Spanned<ParseError>> {
    match self.peek()?.base() {
      Token::Dash | Token::Bang => {
        let operator = self.advance()?;
        let operand = Box::new(self.unary_expression()?);
        Ok(Expression::Unary(UnaryExpression { operator, operand }))
      }
      Token::OpenParen => {
        self.advance()?;
        let expression = self.expression()?;
        self.expect(Token::CloseParen)?;
        Ok(expression)
      }
      _ => Ok(Expression::Literal(self.literal_expression()?)),
    }
  }

  fn literal_expression(
    &mut self,
  ) -> Result<Spanned<Token>, Spanned<ParseError>> {
    match self.peek()?.base() {
      Token::Int
      | Token::Float
      | Token::True
      | Token::False
      | Token::Identifier => Ok(self.advance()?),
      _ => Err(self.advance()?.map(ParseError::UnexpectedToken)),
    }
  }

  fn pattern(&mut self) -> Result<Spanned<Pattern>, Spanned<ParseError>> {
    match self.peek()?.base() {
      Token::Int => {
        let int = self.advance()?;
        let expression = Spanned::new(
          int
            .span()
            .as_str()
            .parse()
            .map(Pattern::Int)
            .or_else(|_| int.span().as_str().parse().map(Pattern::Float))
            .unwrap(),
          int.span().clone(),
        );
        Ok(expression)
      }
      Token::Float => {
        let float = self.advance()?;
        let expression = Spanned::new(
          Pattern::Float(float.span().as_str().parse().unwrap()),
          float.span().clone(),
        );
        Ok(expression)
      }
      Token::True => {
        let bool = self.advance()?;
        let expression = Spanned::new(Pattern::Bool(true), bool.span().clone());
        Ok(expression)
      }
      Token::False => {
        let bool = self.advance()?;
        let expression =
          Spanned::new(Pattern::Bool(false), bool.span().clone());
        Ok(expression)
      }
      Token::Identifier => {
        let identifier = self.advance()?;
        let expression = Spanned::new(
          Pattern::Identifier(identifier.span().as_str().to_string()),
          identifier.span().clone(),
        );
        Ok(expression)
      }
      _ => {
        let token = self.advance()?;
        Err(token.map(ParseError::UnexpectedToken))
      }
    }
  }

  fn synchronize(&mut self) {
    loop {
      if let Ok(Token::Fn | Token::Mod | Token::Eof) =
        self.peek().map(|token| token.base().clone())
      {
        return;
      }
      // TODO: improve error message
      self
        .advance()
        .expect("earlier peek was fine, expected advance to be fine as well");
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

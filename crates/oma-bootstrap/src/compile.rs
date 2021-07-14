use std::{collections::HashMap, fs, io};

use crate::{
  ast,
  ir::{
    AccessExpression, BinaryExpression, BinaryOperator, BindStatement, Block,
    CallExpression, Chunk, ElseBody, Executable, Expression,
    ExpressionStatement, IfExpression, LiteralExpression, ModHeader,
    PackageHeader, Path, Statement, UnaryExpression, UnaryOperator,
    WhileExpression,
  },
  parse::{ParseError, Parser},
  span::Spanned,
  token::Token,
};

#[derive(Debug)]
pub enum CompileError {
  Io(io::Error),
  FileNotFound,
  Parse(Vec<Spanned<ParseError>>),
}

pub struct Compiler {
  identifiers: HashMap<String, usize>,
  chunks: Vec<Chunk>,
}

impl Compiler {
  pub fn new() -> Compiler {
    Compiler {
      identifiers: HashMap::new(),
      chunks: Vec::new(),
    }
  }

  pub fn compile(mut self) -> Result<Executable, CompileError> {
    let source =
      fs::read_to_string("examples/src/lib.oma").map_err(CompileError::Io)?;
    let parser = Parser::new(&source);
    let file = dbg!(parser.parse().map_err(CompileError::Parse)?);

    let package_header = self.package_header(file)?;

    Ok(Executable {
      package_header,
      chunks: self.chunks,
      identifiers: self.identifiers,
    })
  }

  fn package_header(
    &mut self,
    file: ast::File,
  ) -> Result<PackageHeader, CompileError> {
    let mut use_declarations = Vec::new();
    let mut mod_headers = HashMap::new();
    let mut fn_headers = HashMap::new();

    for declaration in file.declarations {
      match declaration {
        ast::Declaration::Use(use_declaration) => {
          use_declarations.extend(self.use_declaration(use_declaration)?);
        }
        ast::Declaration::Mod(mod_declaration) => {
          let name =
            self.add_identifier(mod_declaration.name.span().to_string());
          mod_headers.insert(name, self.mod_header(mod_declaration)?);
        }
        ast::Declaration::Fn(fn_declaration) => {
          let name =
            self.add_identifier(fn_declaration.name.span().to_string());
          fn_headers.insert(name, self.fn_header(fn_declaration)?);
        }
      };
    }

    Ok(PackageHeader {
      package_headers: HashMap::new(),
      use_declarations,
      mod_headers,
      fn_headers,
    })
  }

  fn mod_header(
    &mut self,
    mod_declaration: ast::ModDeclaration,
  ) -> Result<ModHeader, CompileError> {
    let body = if let Some(body) = mod_declaration.body {
      body
    } else {
      let source = fs::read_to_string(format!(
        "examples/src/{}.oma",
        mod_declaration.name.span().as_str()
      ))
      .map_err(CompileError::Io)?;
      let parser = Parser::new(&source);
      parser.parse().map_err(CompileError::Parse)?
    };

    let mut use_declarations = Vec::new();
    let mut mod_headers = HashMap::new();
    let mut fn_headers = HashMap::new();

    for declaration in body.declarations {
      match declaration {
        ast::Declaration::Use(use_declaration) => {
          use_declarations.extend(self.use_declaration(use_declaration)?);
        }
        ast::Declaration::Mod(mod_declaration) => {
          let name =
            self.add_identifier(mod_declaration.name.span().to_string());
          mod_headers.insert(name, self.mod_header(mod_declaration)?);
        }
        ast::Declaration::Fn(fn_declaration) => {
          let name =
            self.add_identifier(fn_declaration.name.span().to_string());
          fn_headers.insert(name, self.fn_header(fn_declaration)?);
        }
      };
    }

    Ok(ModHeader {
      use_declarations,
      mod_headers,
      fn_headers,
    })
  }

  fn fn_header(
    &mut self,
    fn_declaration: ast::FnDeclaration,
  ) -> Result<usize, CompileError> {
    let parameters = fn_declaration
      .parameters
      .iter()
      .map(|token| self.add_identifier(token.span().to_string()))
      .collect();

    let body = self.block(fn_declaration.body)?;

    let chunk = self.add_chunk(Chunk { parameters, body });

    Ok(chunk)
  }

  fn block(&mut self, block: ast::Block) -> Result<Block, CompileError> {
    Ok(Block {
      statements: block
        .statements
        .into_iter()
        .map(|statement| self.statement(statement))
        .collect::<Result<Vec<Vec<Statement>>, CompileError>>()?
        .into_iter()
        .flatten()
        .collect(),
    })
  }

  fn statement(
    &mut self,
    statement: ast::Statement,
  ) -> Result<Vec<Statement>, CompileError> {
    match statement {
      ast::Statement::Bind(bind_statement) => {
        self.bind_statement(bind_statement)
      }
      ast::Statement::Expression(expression_statement) => Ok(
        self
          .expression_statement(expression_statement)?
          .into_iter()
          .map(Statement::Expression)
          .collect(),
      ),
    }
  }

  fn bind_statement(
    &mut self,
    bind_statement: ast::BindStatement,
  ) -> Result<Vec<Statement>, CompileError> {
    match bind_statement.pattern {
      ast::Pattern::Literal(token) => match token.base() {
        Token::Identifier => {
          let name = self.add_identifier(token.span().to_string());
          let expression = expression_or_expressions(
            self.expression(bind_statement.expression)?,
          );
          Ok(vec![Statement::Bind(BindStatement {
            is_mut: bind_statement.is_mut,
            name,
            expression,
          })])
        }
        _ => unreachable!(),
      },
      _ => unreachable!(),
    }
  }

  fn expression_statement(
    &mut self,
    expression_statement: ast::ExpressionStatement,
  ) -> Result<Vec<ExpressionStatement>, CompileError> {
    Ok(
      self
        .expression(expression_statement.expression)?
        .into_iter()
        .map(|expression| ExpressionStatement { expression })
        .collect(),
    )
  }

  fn expression(
    &mut self,
    expression: ast::Expression,
  ) -> Result<Vec<Expression>, CompileError> {
    match expression {
      ast::Expression::Literal(token) => {
        self.literal_expression(token).map(|literal_expression| {
          vec![Expression::Literal(literal_expression)]
        })
      }
      ast::Expression::Path(path) => self
        .path_expression(path)
        .map(|path| vec![Expression::Path(path)]),
      ast::Expression::Access(access_expression) => self
        .access_expression(access_expression)
        .map(|access_expression| vec![Expression::Access(access_expression)]),
      ast::Expression::Call(call_expression) => self
        .call_expression(call_expression)
        .map(|call_expression| vec![Expression::Call(call_expression)]),
      ast::Expression::Unary(unary_expression) => self
        .unary_expression(unary_expression)
        .map(|unary_expression| vec![Expression::Unary(unary_expression)]),
      ast::Expression::Binary(binary_expression) => self
        .binary_expression(binary_expression)
        .map(|binary_expression| vec![Expression::Binary(binary_expression)]),
      ast::Expression::Assign(assign_expression) => self
        .assign_expression(assign_expression)
        .map(|assign_expression| vec![]),
      ast::Expression::If(if_expression) => self
        .if_expression(if_expression)
        .map(|if_expression| vec![Expression::If(if_expression)]),
      ast::Expression::While(while_expression) => self
        .while_expression(while_expression)
        .map(|while_expression| vec![Expression::While(while_expression)]),
    }
  }

  fn literal_expression(
    &mut self,
    token: Spanned<Token>,
  ) -> Result<LiteralExpression, CompileError> {
    match token.base() {
      Token::Int => Ok(LiteralExpression::Int(
        token.span().as_str().parse().unwrap(),
      )),
      Token::Float => Ok(LiteralExpression::Float(
        token.span().as_str().parse().unwrap(),
      )),
      Token::True => Ok(LiteralExpression::Bool(true)),
      Token::False => Ok(LiteralExpression::Bool(false)),
      Token::Identifier => Ok(LiteralExpression::Identifier(
        self.add_identifier(token.span().to_string()),
      )),
      _ => unreachable!(),
    }
  }

  fn path_expression(&mut self, path: ast::Path) -> Result<Path, CompileError> {
    Ok(Path {
      components: path
        .components
        .into_iter()
        .map(|token| self.add_identifier(token.span().to_string()))
        .collect(),
    })
  }

  fn access_expression(
    &mut self,
    access_expression: ast::AccessExpression,
  ) -> Result<AccessExpression, CompileError> {
    let receiver = Box::new(expression_or_expressions(
      self.expression(*access_expression.receiver)?,
    ));

    Ok(AccessExpression {
      receiver,
      field: self.add_identifier(access_expression.field.span().to_string()),
    })
  }

  fn call_expression(
    &mut self,
    call_expression: ast::CallExpression,
  ) -> Result<CallExpression, CompileError> {
    let receiver = Box::new(expression_or_expressions(
      self.expression(*call_expression.receiver)?,
    ));

    let arguments = call_expression
      .arguments
      .into_iter()
      .map(|expression| self.expression(expression))
      .collect::<Result<Vec<Vec<Expression>>, CompileError>>()?
      .into_iter()
      .map(expression_or_expressions)
      .collect();

    Ok(CallExpression {
      receiver,
      arguments,
    })
  }

  fn unary_expression(
    &mut self,
    unary_expression: ast::UnaryExpression,
  ) -> Result<UnaryExpression, CompileError> {
    let operator = match unary_expression.operator.base() {
      Token::Dash => UnaryOperator::Negate,
      Token::Bang => UnaryOperator::Not,
      _ => unreachable!(),
    };

    let operand = Box::new(expression_or_expressions(
      self.expression(*unary_expression.operand)?,
    ));

    Ok(UnaryExpression { operator, operand })
  }

  fn binary_expression(
    &mut self,
    binary_expression: ast::BinaryExpression,
  ) -> Result<BinaryExpression, CompileError> {
    let left_operand = Box::new(expression_or_expressions(
      self.expression(*binary_expression.left_operand)?,
    ));

    let operator = match binary_expression.operator.base() {
      Token::AmpAmp => BinaryOperator::And,
      Token::PipePipe => BinaryOperator::Or,
      Token::Greater => BinaryOperator::Greater,
      Token::GreaterEqual => BinaryOperator::GreaterEqual,
      Token::Less => BinaryOperator::Less,
      Token::LessEqual => BinaryOperator::LessEqual,
      Token::EqualEqual => BinaryOperator::Equal,
      Token::Plus => BinaryOperator::Add,
      Token::Dash => BinaryOperator::Subtract,
      Token::Star => BinaryOperator::Multiply,
      Token::Slash => BinaryOperator::Divide,
      _ => unreachable!(),
    };

    let right_operand = Box::new(expression_or_expressions(
      self.expression(*binary_expression.right_operand)?,
    ));

    Ok(BinaryExpression {
      left_operand,
      operator,
      right_operand,
    })
  }

  fn assign_expression(
    &mut self,
    _assign_expression: ast::AssignExpression,
  ) -> Result<Expression, CompileError> {
    unimplemented!();
  }

  fn if_expression(
    &mut self,
    if_expression: ast::IfExpression,
  ) -> Result<IfExpression, CompileError> {
    let condition = Box::new(expression_or_expressions(
      self.expression(*if_expression.condition)?,
    ));
    let body = self.block(if_expression.body)?;
    let else_body = if_expression
      .else_body
      .map(|else_body| self.else_body(else_body))
      .transpose()?;

    Ok(IfExpression {
      condition,
      body,
      else_body,
    })
  }

  fn else_body(
    &mut self,
    else_body: ast::ElseBody,
  ) -> Result<ElseBody, CompileError> {
    match else_body {
      ast::ElseBody::Else(block) => self.block(block).map(ElseBody::Else),
      ast::ElseBody::If(if_expression) => self
        .if_expression(*if_expression)
        .map(Box::new)
        .map(ElseBody::If),
    }
  }

  fn while_expression(
    &mut self,
    while_expression: ast::WhileExpression,
  ) -> Result<WhileExpression, CompileError> {
    let condition = Box::new(expression_or_expressions(
      self.expression(*while_expression.condition)?,
    ));
    let body = self.block(while_expression.body)?;

    Ok(WhileExpression { condition, body })
  }

  fn use_declaration(
    &mut self,
    use_declaration: ast::UseDeclaration,
  ) -> Result<Vec<Path>, CompileError> {
    Ok(
      use_declaration
        .trees
        .into_iter()
        .map(|use_tree| self.use_tree(use_tree))
        .collect::<Result<Vec<Vec<Vec<usize>>>, CompileError>>()?
        .into_iter()
        .flatten()
        .map(|components| Path { components })
        .collect(),
    )
  }

  fn use_tree(
    &mut self,
    use_tree: ast::UseTree,
  ) -> Result<Vec<Vec<usize>>, CompileError> {
    match use_tree {
      ast::UseTree::Branch(branch) => {
        let component =
          self.add_identifier(branch.component.span().to_string());

        Ok(
          branch
            .subtrees
            .into_iter()
            .map(|use_tree| self.use_tree(use_tree))
            .collect::<Result<Vec<Vec<Vec<usize>>>, CompileError>>()?
            .into_iter()
            .flatten()
            .map(|components| {
              let mut components = components.clone();
              components.push(component);
              components
            })
            .collect(),
        )
      }
      ast::UseTree::Leaf(token) => {
        Ok(vec![vec![self.add_identifier(token.span().to_string())]])
      }
    }
  }

  fn add_chunk(&mut self, chunk: Chunk) -> usize {
    self.chunks.push(chunk);
    self.chunks.len() - 1
  }

  fn add_identifier(&mut self, identifier: String) -> usize {
    if let Some(identifier) = self.identifiers.get(&identifier) {
      *identifier
    } else {
      self.identifiers.insert(identifier, self.identifiers.len());
      self.identifiers.len() - 1
    }
  }
}

fn expression_or_expressions(mut expressions: Vec<Expression>) -> Expression {
  if expressions.len() == 1 {
    expressions.pop().unwrap()
  } else {
    Expression::Block(Block {
      statements: expressions
        .into_iter()
        .map(|expression| {
          Statement::Expression(ExpressionStatement { expression })
        })
        .collect(),
    })
  }
}

use oma::{
  executable::{Chunk, Constant},
  instruction::Instruction,
};

use crate::{
  ast::{
    BinaryExpression, BindStatement, Block, ElseBody, Expression,
    ExpressionStatement, IfExpression, Pattern, Statement, UnaryExpression,
    WhileExpression,
  },
  span::Spanned,
  token::Token,
};

pub struct Generator {}

impl Generator {
  pub fn new() -> Generator {
    Generator {}
  }

  pub fn generate(mut self, block: Block) -> Chunk {
    let mut chunk = Chunk::new();
    self.block(&mut chunk, block);
    chunk.emit(Instruction::Return);
    chunk
  }

  fn block(&mut self, chunk: &mut Chunk, block: Block) {
    let requires_unit_return = block
      .statements
      .last()
      .map(|statement| {
        if let Statement::Expression(expression_statement) = statement {
          expression_statement.has_semicolon
        } else {
          true
        }
      })
      .unwrap_or(true);
    for statement in block.statements {
      self.statement(chunk, statement)
    }
    if requires_unit_return {
      chunk.emit(Instruction::PushUnit);
    }
  }

  fn statement(&mut self, chunk: &mut Chunk, statement: Statement) {
    match statement {
      Statement::Bind(bind_statement) => {
        self.bind_statement(chunk, bind_statement);
      }
      Statement::Expression(expression_statement) => {
        self.expression_statement(chunk, expression_statement);
      }
    }
  }

  fn bind_statement(
    &mut self,
    chunk: &mut Chunk,
    bind_statement: BindStatement,
  ) {
    self.expression(chunk, *bind_statement.expression);
    if let Pattern::Literal(token) = bind_statement.pattern {
      if let Token::Identifier = token.base() {
        chunk.add_local(token.span().as_str().to_string());
        return;
      }
    }
    panic!("cannot assign to non-identifier");
  }

  fn expression_statement(
    &mut self,
    chunk: &mut Chunk,
    expression_statement: ExpressionStatement,
  ) {
    self.expression(chunk, expression_statement.expression);
    if expression_statement.has_semicolon {
      chunk.emit(Instruction::Pop);
    }
  }

  fn expression(&mut self, chunk: &mut Chunk, expression: Expression) {
    match expression {
      Expression::Assign(_) => {}
      Expression::Path(_) => {}
      Expression::Call(_) => {}
      Expression::Access(_) => {}
      Expression::Binary(binary_expression) => {
        self.binary_expression(chunk, binary_expression)
      }
      Expression::Unary(unary_expression) => {
        self.unary_expression(chunk, unary_expression)
      }
      Expression::Literal(literal_expression) => {
        self.literal_expression(chunk, literal_expression)
      }
      Expression::If(if_expression) => self.if_expression(chunk, if_expression),
      Expression::While(while_expression) => {
        self.while_expression(chunk, while_expression)
      }
    }
  }

  fn binary_expression(
    &mut self,
    chunk: &mut Chunk,
    binary_expression: BinaryExpression,
  ) {
    self.expression(chunk, *binary_expression.left_operand);
    self.expression(chunk, *binary_expression.right_operand);

    let instruction = match binary_expression.operator.base() {
      Token::Plus => Instruction::Add,
      Token::Dash => Instruction::Subtract,
      Token::Star => Instruction::Multiply,
      Token::Slash => Instruction::Divide,
      Token::Greater => Instruction::Greater,
      Token::GreaterEqual => Instruction::GreaterEqual,
      Token::Less => Instruction::Less,
      Token::LessEqual => Instruction::LessEqual,
      Token::EqualEqual => Instruction::Equal,
      Token::BangEqual => Instruction::NotEqual,
      Token::AmpAmp => Instruction::And,
      Token::PipePipe => Instruction::Or,
      _ => unreachable!("invalid operator in binary expression"),
    };
    chunk.emit(instruction);
  }

  fn unary_expression(
    &mut self,
    chunk: &mut Chunk,
    unary_expression: UnaryExpression,
  ) {
    self.expression(chunk, *unary_expression.operand);

    let instruction = match unary_expression.operator.base() {
      Token::Dash => Instruction::Negate,
      Token::Bang => Instruction::Not,
      _ => unreachable!("invalid operator in unary expression"),
    };
    chunk.emit(instruction);
  }

  fn literal_expression(&mut self, chunk: &mut Chunk, token: Spanned<Token>) {
    let constant = match token.base() {
      Token::Int => Constant::Int(token.span().as_str().parse().unwrap()),
      Token::Float => Constant::Float(token.span().as_str().parse().unwrap()),
      Token::True => Constant::Bool(true),
      Token::False => Constant::Bool(false),
      Token::Identifier => {
        let identifier = token.span().as_str().to_string();
        let index = chunk
          .local(&identifier)
          .expect(format!("local '{}' not defined", identifier).as_str())
          as u64;
        chunk.emit(Instruction::PushLocal);
        chunk.emit_bytes(index.to_le_bytes());
        return;
      }
      _ => unreachable!("invalid token passed to literal expression"),
    };
    let index = chunk.add_constant(constant) as u64;
    chunk.emit(Instruction::PushConstant);
    chunk.emit_bytes(index.to_le_bytes());
  }

  fn if_expression(&mut self, chunk: &mut Chunk, if_expression: IfExpression) {
    self.expression(chunk, *if_expression.condition);

    chunk.emit(Instruction::JumpIf);
    let jump_if_offset = chunk.emit_bytes(0u64.to_le_bytes());

    if let Some(else_body) = if_expression.else_body {
      match else_body {
        ElseBody::If(if_expression) => {
          self.if_expression(chunk, *if_expression)
        }
        ElseBody::Else(block) => self.block(chunk, block),
      }
    }

    chunk.emit(Instruction::Jump);
    let jump_offset = chunk.emit_bytes(0u64.to_le_bytes());

    self.block(chunk, if_expression.body);

    let u64_bytes_len = 0u64.to_le_bytes().len() as u64;
    chunk.patch_bytes(
      jump_if_offset,
      (jump_offset as u64 + u64_bytes_len).to_le_bytes(),
    );
    chunk.patch_bytes(jump_offset, (chunk.code().len() as u64).to_le_bytes());
  }

  fn while_expression(
    &mut self,
    chunk: &mut Chunk,
    while_expression: WhileExpression,
  ) {
    self.expression(chunk, *while_expression.condition);

    chunk.emit(Instruction::Not);
    chunk.emit(Instruction::JumpIf);
    let jump_if_offset = chunk.emit_bytes(0u64.to_le_bytes());

    self.block(chunk, while_expression.body);

    chunk.emit(Instruction::Pop);

    let u64_bytes_len = 0u64.to_le_bytes().len() as u64;
    chunk.emit(Instruction::Jump);
    chunk.emit_bytes((jump_if_offset as u64 + u64_bytes_len).to_le_bytes());

    chunk
      .patch_bytes(jump_if_offset, (chunk.code().len() as u64).to_le_bytes());

    chunk.emit(Instruction::PushUnit);
  }
}

#[cfg(test)]
mod tests {
  use oma::{
    executable::{Chunk, Constant},
    instruction::Instruction,
  };

  use crate::parse::Parser;

  use super::Generator;

  #[test]
  fn addition() {
    let parser = Parser::new("{ 1 + 2 + 3; }");
    let block = parser.parse();
    let generator = Generator::new();

    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(Constant::Int(1));
    chunk.emit(Instruction::PushConstant);
    chunk.emit_bytes(constant.to_le_bytes());

    let constant = chunk.add_constant(Constant::Int(2));
    chunk.emit(Instruction::PushConstant);
    chunk.emit_bytes(constant.to_le_bytes());

    chunk.emit(Instruction::Add);

    let constant = chunk.add_constant(Constant::Int(3));
    chunk.emit(Instruction::PushConstant);
    chunk.emit_bytes(constant.to_le_bytes());

    chunk.emit(Instruction::Add);

    chunk.emit(Instruction::Pop);

    chunk.emit(Instruction::PushUnit);

    chunk.emit(Instruction::Return);

    assert_eq!(generator.generate(block), chunk);
  }
}

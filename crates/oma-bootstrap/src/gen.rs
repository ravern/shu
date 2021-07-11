use oma::{
  executable::{Chunk, Constant},
  instruction::Instruction,
};

use crate::{
  ast::{BinaryExpression, Expression, LiteralExpression, UnaryExpression},
  span::Spanned,
  token::Token,
};

pub struct Generator {}

impl Generator {
  pub fn new() -> Generator {
    Generator {}
  }

  pub fn generate(mut self, expression: Spanned<Expression>) -> Chunk {
    let mut chunk = Chunk::new();
    self.expression(&mut chunk, expression.unwrap());
    chunk.emit(Instruction::Return);
    chunk
  }

  fn expression(&mut self, chunk: &mut Chunk, expression: Expression) {
    match expression {
      Expression::Binary(binary_expression) => {
        self.binary_expression(chunk, binary_expression)
      }
      Expression::Unary(unary_expression) => {
        self.unary_expression(chunk, unary_expression)
      }
      Expression::Literal(literal_expression) => {
        self.literal_expression(chunk, literal_expression)
      }
    }
  }

  fn binary_expression(
    &mut self,
    chunk: &mut Chunk,
    binary_expression: BinaryExpression,
  ) {
    self.expression(chunk, binary_expression.left_operand.unwrap());
    self.expression(chunk, binary_expression.right_operand.unwrap());

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
    self.expression(chunk, unary_expression.operand.unwrap());

    let instruction = match unary_expression.operator.base() {
      Token::Dash => Instruction::Negate,
      Token::Bang => Instruction::Not,
      _ => unreachable!("invalid operator in unary expression"),
    };
    chunk.emit(instruction);
  }

  fn literal_expression(
    &mut self,
    chunk: &mut Chunk,
    literal_expression: LiteralExpression,
  ) {
    let constant = match literal_expression {
      LiteralExpression::Int(int) => Constant::Int(int),
      LiteralExpression::Float(float) => Constant::Float(float),
      LiteralExpression::Bool(bool) => Constant::Bool(bool),
    };
    let constant = chunk.add_constant(constant);
    chunk.emit(Instruction::PushConstant);
    chunk.emit_bytes(constant.to_le_bytes());
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
    let parser = Parser::new("1 + 2 + 3");
    let expression = parser.parse();
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

    chunk.emit(Instruction::Return);

    assert_eq!(generator.generate(expression), chunk);
  }
}

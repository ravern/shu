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
    chunk.add_instruction(Instruction::Return as u64);
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
      Token::Hyphen => Instruction::Subtract,
      Token::Asterisk => Instruction::Multiply,
      Token::Slash => Instruction::Divide,
      _ => unreachable!("invalid operator in binary expression"),
    };
    chunk.add_instruction(instruction as u64);
  }

  fn unary_expression(
    &mut self,
    chunk: &mut Chunk,
    unary_expression: UnaryExpression,
  ) {
    self.expression(chunk, unary_expression.operand.unwrap());

    let instruction = match unary_expression.operator.base() {
      Token::Hyphen => Instruction::Negate,
      _ => unreachable!("invalid operator in unary expression"),
    };
    chunk.add_instruction(instruction as u64);
  }

  fn literal_expression(
    &mut self,
    chunk: &mut Chunk,
    literal_expression: LiteralExpression,
  ) {
    let constant = match literal_expression {
      LiteralExpression::Int(int) => Constant::Int(int),
      LiteralExpression::Float(float) => Constant::Float(float),
    };
    let constant = chunk.add_constant(constant);
    chunk.add_instruction(Instruction::PushConstant as u64);
    chunk.add_instruction(constant as u64);
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
    chunk.add_instruction(Instruction::PushConstant as u64);
    chunk.add_instruction(constant as u64);

    let constant = chunk.add_constant(Constant::Int(2));
    chunk.add_instruction(Instruction::PushConstant as u64);
    chunk.add_instruction(constant as u64);

    chunk.add_instruction(Instruction::Add as u64);

    let constant = chunk.add_constant(Constant::Int(3));
    chunk.add_instruction(Instruction::PushConstant as u64);
    chunk.add_instruction(constant as u64);

    chunk.add_instruction(Instruction::Add as u64);

    chunk.add_instruction(Instruction::Return as u64);

    assert_eq!(generator.generate(dbg!(expression)), chunk);
  }
}

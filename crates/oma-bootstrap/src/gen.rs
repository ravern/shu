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

    match binary_expression.operator.base() {
      Token::Plus => {
        chunk.add_instruction(Instruction::Add as u64);
      }
      Token::Hyphen => {
        chunk.add_instruction(Instruction::Sub as u64);
      }
      _ => unreachable!("invalid operator in binary expression"),
    }
  }

  fn unary_expression(
    &mut self,
    chunk: &mut Chunk,
    unary_expression: UnaryExpression,
  ) {
    self.expression(chunk, unary_expression.operand.unwrap());

    match unary_expression.operator.base() {
      _ => unreachable!("invalid operator in unary expression"),
    }
  }

  fn literal_expression(
    &mut self,
    chunk: &mut Chunk,
    literal_expression: LiteralExpression,
  ) {
    match literal_expression {
      LiteralExpression::Int(int) => {
        let constant = chunk.add_constant(Constant::Int(int));
        chunk.add_instruction(Instruction::Constant as u64);
        chunk.add_instruction(constant as u64);
      }
      LiteralExpression::Float(float) => {
        let constant = chunk.add_constant(Constant::Float(float));
        chunk.add_instruction(Instruction::Constant as u64);
        chunk.add_instruction(constant as u64);
      }
    }
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
    chunk.add_instruction(Instruction::Constant as u64);
    chunk.add_instruction(constant as u64);

    let constant = chunk.add_constant(Constant::Int(2));
    chunk.add_instruction(Instruction::Constant as u64);
    chunk.add_instruction(constant as u64);

    chunk.add_instruction(Instruction::Add as u64);

    let constant = chunk.add_constant(Constant::Int(3));
    chunk.add_instruction(Instruction::Constant as u64);
    chunk.add_instruction(constant as u64);

    chunk.add_instruction(Instruction::Add as u64);

    chunk.add_instruction(Instruction::Return as u64);

    assert_eq!(generator.generate(dbg!(expression)), chunk);
  }
}

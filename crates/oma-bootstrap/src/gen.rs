use oma::{
  executable::{Chunk, Constant},
  instruction::Instruction,
};

use crate::{
  ast::{
    BinaryExpression, BindStatement, Block, Expression, LiteralExpression,
    Statement, UnaryExpression,
  },
  span::Spanned,
  token::Token,
};

pub struct Generator {}

impl Generator {
  pub fn new() -> Generator {
    Generator {}
  }

  pub fn generate(mut self, block: Spanned<Block>) -> Chunk {
    let mut chunk = Chunk::new();
    self.block(&mut chunk, block.unwrap());
    chunk.emit(Instruction::Return);
    chunk
  }

  fn block(&mut self, chunk: &mut Chunk, block: Block) {
    for statement in block.statements {
      self.statement(chunk, statement.unwrap())
    }
  }

  fn statement(&mut self, chunk: &mut Chunk, statement: Statement) {
    match statement {
      Statement::Bind(bind_statement) => {
        self.bind_statement(chunk, bind_statement);
      }
      Statement::Expression(expression_statement) => {
        self.expression(chunk, expression_statement.expression.unwrap());
        if expression_statement.semicolon_token.is_some() {
          chunk.emit(Instruction::Pop);
        }
      }
    }
  }

  fn bind_statement(
    &mut self,
    chunk: &mut Chunk,
    bind_statement: BindStatement,
  ) {
    self.expression(chunk, bind_statement.expression.unwrap());
    if let Expression::Literal(LiteralExpression::Identifier(identifier)) =
      bind_statement.pattern.unwrap()
    {
      chunk.add_local(identifier);
    } else {
      panic!("cannot assign to non-identifier");
    }
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
      LiteralExpression::Identifier(identifier) => {
        let index = chunk
          .local(&identifier)
          .expect(&format!("local '{}' not defined", identifier))
          as u64;
        chunk.emit(Instruction::PushLocal);
        chunk.emit_bytes(index.to_le_bytes());
        return;
      }
    };
    let constant = chunk.add_constant(constant) as u64;
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

    chunk.emit(Instruction::Return);

    assert_eq!(generator.generate(block), chunk);
  }
}

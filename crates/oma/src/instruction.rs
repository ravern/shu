use std::fmt;

use num_derive::FromPrimitive;

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq)]
pub enum Instruction {
  PushConstant = 0,
  PushLocal,
  PushUnit,
  Pop,
  Add,
  Subtract,
  Multiply,
  Divide,
  Negate,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  Equal,
  NotEqual,
  Not,
  And,
  Or,
  Return,
}

impl fmt::Display for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Instruction::PushConstant => write!(f, "PushConstant"),
      Instruction::PushLocal => write!(f, "PushLocal"),
      Instruction::PushUnit => write!(f, "PushUnit"),
      Instruction::Pop => write!(f, "Pop"),
      Instruction::Add => write!(f, "Add"),
      Instruction::Subtract => write!(f, "Subtract"),
      Instruction::Multiply => write!(f, "Multiply"),
      Instruction::Divide => write!(f, "Divide"),
      Instruction::Negate => write!(f, "Negate"),
      Instruction::Greater => write!(f, "Greater"),
      Instruction::GreaterEqual => write!(f, "GreaterEqual"),
      Instruction::Less => write!(f, "Less"),
      Instruction::LessEqual => write!(f, "LessEqual"),
      Instruction::Equal => write!(f, "Equal"),
      Instruction::NotEqual => write!(f, "NotEqual"),
      Instruction::Not => write!(f, "Not"),
      Instruction::And => write!(f, "And"),
      Instruction::Or => write!(f, "Or"),
      Instruction::Return => write!(f, "Return"),
    }
  }
}

use num_derive::FromPrimitive;

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq)]
pub enum Instruction {
  PushConstant = 0,
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

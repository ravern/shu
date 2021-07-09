use num_derive::FromPrimitive;

#[repr(u64)]
#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq)]
pub enum Instruction {
  PushConstant = 1u64,
  Add,
  Subtract,
  Multiply,
  Divide,
  Negate,
  Return,
}

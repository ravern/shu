use num_derive::FromPrimitive;

#[repr(u64)]
#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq)]
pub enum Instruction {
  Constant = 1u64,
  Add,
  Return,
}

use std::cell::RefCell;

use crate::{op::*, value::Value};

pub use crate::{error::Error, executable::Executable};

mod error;
mod executable;
pub mod op;
pub mod value;

pub struct Machine {
  context: RefCell<Context>,
}

impl Machine {
  pub fn execute(&self, executable: Executable) -> Result<(), Error> {
    loop {
      let mut context = self.context.borrow_mut();
      let current_instruction = context.current_instruction;

      let op = executable
        .op(current_instruction)
        .ok_or(Error::SegmentationFault)?;
      match op {
        OP_NOOP => {}
        OP_PUSH => {
          let index = usize::from_le_bytes([0u8; 8]);
          let atom =
            executable.constant(index).ok_or(Error::SegmentationFault)?;
          context.push(Value::Atom(atom))
        }
        op => return Err(Error::InvalidOp(op)),
      }

      context.step();
    }
  }
}

struct Context {
  current_instruction: usize,
  stack: Vec<Value>,
}

impl Context {
  pub fn step(&mut self) {
    self.current_instruction += 1;
  }

  pub fn push(&mut self, value: Value) {
    self.stack.push(value);
  }
}

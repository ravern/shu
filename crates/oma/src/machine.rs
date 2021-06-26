use std::{cell::RefCell, rc::Rc};

use crate::{
  executable::{Chunk, Executable},
  op::*,
  value::Value,
};

pub struct Machine {
  context: RefCell<Context>,
}

impl Machine {
  pub fn new() -> Machine {
    Machine {
      context: RefCell::new(Context::Idle),
    }
  }

  pub fn execute(&self, executable: &[u8]) -> Result<(), Error> {
    loop {
      let mut context = self.context.borrow_mut();
      let current_instruction = context.current_instruction;

      let op = executable
        .op(current_instruction)
        .ok_or(Error::SegmentationFault)?;
      match op {
        OP_PUSH => {
          let index = usize::from_le_bytes([0u8; 8]);
          let atom =
            executable.constant(index).ok_or(Error::SegmentationFault)?;
          context.push(Value::Atom(atom))
        }
        op => return Err(Error::UnsupportedOp(op)),
      }

      context.step();
    }
  }
}

pub enum Context {
  Idle,
  Executing {
    executable: Executable,
    frames: Vec<Frame>,
    stack: Vec<Value>,
  },
}

impl Context {
  pub fn step(&mut self) {
    match self {
      Context::Idle => {},
      Context::Executing {
        frames:
      }
    }
    self.current_instruction += 1;
  }

  pub fn push(&mut self, value: Value) {
    self.stack.push(value);
  }

  pub fn 
}

pub struct Frame {
  current_instruction: usize,
  chunk: Rc<Chunk>,
}

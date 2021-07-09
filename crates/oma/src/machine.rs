use num_traits::FromPrimitive;

use crate::{executable::Chunk, instruction::Instruction, value::Value};

#[derive(Debug)]
pub enum Error {
  SegmentationFault(usize),
  InvalidInstruction(u64),
  InvalidConstant(u64),
  EmptyStack,
}

pub struct Machine {
  current: usize,
  stack: Vec<Value>,
}

impl Machine {
  pub fn new() -> Machine {
    Machine {
      current: 0,
      stack: Vec::with_capacity(32),
    }
  }

  pub fn execute(&mut self, chunk: &Chunk) -> Result<Value, Error> {
    self.current = 0;
    self.stack = Vec::new();

    loop {
      let instruction = chunk
        .instruction(self.current)
        .ok_or(Error::SegmentationFault(self.current))?;
      let instruction = Instruction::from_u64(instruction)
        .ok_or(Error::InvalidInstruction(instruction))?;
      self.current += 1;

      match instruction {
        Instruction::Constant => {
          let constant = chunk
            .instruction(self.current)
            .ok_or(Error::SegmentationFault(self.current))?;
          self.current += 1;
          let constant = chunk
            .constant(constant as usize)
            .ok_or(Error::InvalidConstant(constant))?;
          self.push(constant.into());
        }
        Instruction::Add => {
          let right = self.pop()?;
          let left = self.pop()?;
          let result = match (left, right) {
            (Value::Int(left), Value::Int(right)) => Value::Int(left + right),
            (Value::Float(left), Value::Int(right)) => {
              Value::Float(left + right as f64)
            }
            (Value::Int(left), Value::Float(right)) => {
              Value::Float(left as f64 + right)
            }
            (Value::Float(left), Value::Float(right)) => {
              Value::Float(left + right)
            }
          };
          self.push(result);
        }
        Instruction::Sub => {
          let right = self.pop()?;
          let left = self.pop()?;
          let result = match (left, right) {
            (Value::Int(left), Value::Int(right)) => Value::Int(left - right),
            (Value::Float(left), Value::Int(right)) => {
              Value::Float(left - right as f64)
            }
            (Value::Int(left), Value::Float(right)) => {
              Value::Float(left as f64 - right)
            }
            (Value::Float(left), Value::Float(right)) => {
              Value::Float(left - right)
            }
          };
          self.push(result);
        }
        Instruction::Return => {
          return self.pop();
        }
      };
    }
  }

  fn push(&mut self, value: Value) {
    self.stack.push(value);
  }

  fn pop(&mut self) -> Result<Value, Error> {
    self.stack.pop().ok_or(Error::EmptyStack)
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    executable::{Chunk, Constant},
    instruction::Instruction,
    value::Value,
  };

  use super::Machine;

  #[test]
  fn addition() {
    let mut machine = Machine::new();

    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(Constant::Float(1.0));
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

    assert_eq!(machine.execute(&chunk).unwrap(), Value::Float(6.0));
  }
}

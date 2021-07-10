use std::{
  cmp::{PartialEq, PartialOrd},
  ops::{Add, Div, Mul, Neg, Sub},
};

use num_traits::FromPrimitive;

use crate::{executable::Chunk, instruction::Instruction, value::Value};

#[derive(Debug)]
pub enum Error {
  SegmentationFault(usize),
  InvalidInstruction(u64),
  InvalidConstant(u64),
  InvalidType,
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
    macro_rules! arithmetic {
      ($op:ident) => {
        let right = self.pop()?;
        let left = self.pop()?;
        let result = match (left, right) {
          (Value::Int(left), Value::Int(right)) => Value::Int(left.$op(right)),
          (Value::Float(left), Value::Int(right)) => {
            Value::Float(left.$op(right as f64))
          }
          (Value::Int(left), Value::Float(right)) => {
            Value::Float((left as f64).$op(right))
          }
          (Value::Float(left), Value::Float(right)) => {
            Value::Float(left.$op(right))
          }
          _ => return Err(Error::InvalidType),
        };
        self.push(result);
      };
    }

    macro_rules! equality {
      ($op:ident) => {
        let right = self.pop()?;
        let left = self.pop()?;
        let result = match (left, right) {
          (Value::Int(left), Value::Int(right)) => {
            Value::Bool(left.$op(&right))
          }
          (Value::Float(left), Value::Int(right)) => {
            Value::Bool(left.$op(&(right as f64)))
          }
          (Value::Int(left), Value::Float(right)) => {
            Value::Bool((left as f64).$op(&right))
          }
          (Value::Float(left), Value::Float(right)) => {
            Value::Bool(left.$op(&right))
          }
          (Value::Bool(left), Value::Bool(right)) => {
            Value::Bool(left.$op(&right))
          }
          _ => return Err(Error::InvalidType),
        };
        self.push(result);
      };
    }

    macro_rules! comparison {
      ($op:ident) => {
        let right = self.pop()?;
        let left = self.pop()?;
        let result = match (left, right) {
          (Value::Int(left), Value::Int(right)) => {
            Value::Bool(left.$op(&right))
          }
          (Value::Float(left), Value::Int(right)) => {
            Value::Bool(left.$op(&(right as f64)))
          }
          (Value::Int(left), Value::Float(right)) => {
            Value::Bool((left as f64).$op(&right))
          }
          (Value::Float(left), Value::Float(right)) => {
            Value::Bool(left.$op(&right))
          }
          _ => return Err(Error::InvalidType),
        };
        self.push(result);
      };
    }

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
        Instruction::PushConstant => {
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
          arithmetic!(add);
        }
        Instruction::Subtract => {
          arithmetic!(sub);
        }
        Instruction::Multiply => {
          arithmetic!(mul);
        }
        Instruction::Divide => {
          arithmetic!(div);
        }
        Instruction::Negate => {
          let operand = self.pop()?;
          let result = match operand {
            Value::Int(int) => Value::Int(-int),
            Value::Float(float) => Value::Float(-float),
            _ => return Err(Error::InvalidType),
          };
          self.push(result);
        }
        Instruction::Greater => {
          comparison!(gt);
        }
        Instruction::GreaterEqual => {
          comparison!(ge);
        }
        Instruction::Less => {
          comparison!(lt);
        }
        Instruction::LessEqual => {
          comparison!(le);
        }
        Instruction::Equal => {
          equality!(eq);
        }
        Instruction::NotEqual => {
          equality!(ne);
        }
        Instruction::Not => {
          let operand = self.pop()?;
          let result = match operand {
            Value::Bool(bool) => Value::Bool(!bool),
            _ => return Err(Error::InvalidType),
          };
          self.push(result);
        }
        Instruction::And => {
          let right = self.pop()?;
          let left = self.pop()?;
          let result = match (left, right) {
            (Value::Bool(left), Value::Bool(right)) => {
              Value::Bool(left && right)
            }
            _ => return Err(Error::InvalidType),
          };
          self.push(result);
        }
        Instruction::Or => {
          let right = self.pop()?;
          let left = self.pop()?;
          let result = match (left, right) {
            (Value::Bool(left), Value::Bool(right)) => {
              Value::Bool(left || right)
            }
            _ => return Err(Error::InvalidType),
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

    assert_eq!(machine.execute(&chunk).unwrap(), Value::Float(6.0));
  }
}

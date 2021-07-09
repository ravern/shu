use std::io::{self, Read};

#[derive(Debug)]
pub enum ParseError {
  InvalidConstant(u64),
  InvalidInstruction(u64),
  Io(io::Error),
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
  constants: Vec<Constant>,
  instructions: Vec<u64>,
}

impl Chunk {
  pub fn new() -> Chunk {
    Chunk {
      constants: Vec::new(),
      instructions: Vec::new(),
    }
  }

  pub fn add_constant(&mut self, constant: Constant) -> usize {
    self.constants.push(constant);
    self.constants.len() - 1
  }

  pub fn add_instruction(&mut self, instruction: u64) -> usize {
    self.instructions.push(instruction);
    self.instructions.len() - 1
  }

  pub fn instruction(&self, offset: usize) -> Option<u64> {
    self.instructions.get(offset).copied()
  }

  pub fn constant(&self, offset: usize) -> Option<Constant> {
    self.constants.get(offset).cloned()
  }

  pub fn len(&self) -> usize {
    self.instructions.len()
  }

  pub fn from_bytes<R>(r: &mut R) -> Result<Chunk, ParseError>
  where
    R: Read,
  {
    let mut chunk = Chunk::new();

    let constants_len = read_u64(r)?;
    for _ in 0..constants_len {
      chunk.constants.push(Constant::from_bytes(r)?);
    }

    let instructions_len = read_u64(r)?;
    for _ in 0..instructions_len {
      let instruction = read_u64(r)?;
      chunk.instructions.push(instruction);
    }

    Ok(chunk)
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    let mut bytes = Vec::new();

    let constants_len = self.constants.len() as u64;
    bytes.extend(constants_len.to_le_bytes());
    for constant in self.constants.iter() {
      bytes.extend(constant.to_bytes());
    }

    let instructions_len = self.instructions.len() as u64;
    bytes.extend(instructions_len.to_le_bytes());
    for instruction in self.instructions.iter() {
      bytes.extend(instruction.to_le_bytes());
    }

    bytes
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
  Int(i64),
  Float(f64),
}

impl Constant {
  pub fn from_bytes<R>(r: &mut R) -> Result<Constant, ParseError>
  where
    R: Read,
  {
    let constant = match read_u64(r)? {
      1 => {
        let mut bytes = [0u8; 8];
        r.read_exact(&mut bytes).map_err(ParseError::Io)?;
        Constant::Int(i64::from_le_bytes(bytes))
      }
      2 => {
        let mut bytes = [0u8; 8];
        r.read_exact(&mut bytes).map_err(ParseError::Io)?;
        Constant::Float(f64::from_le_bytes(bytes))
      }
      constant => {
        return Err(ParseError::InvalidConstant(constant));
      }
    };

    Ok(constant)
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    let mut bytes = Vec::new();

    match self {
      Constant::Int(int) => {
        bytes.extend(1u64.to_le_bytes());
        bytes.extend(int.to_le_bytes());
      }
      Constant::Float(float) => {
        bytes.extend(2u64.to_le_bytes());
        bytes.extend(float.to_le_bytes());
      }
    }

    bytes
  }
}

fn read_u64<R>(r: &mut R) -> Result<u64, ParseError>
where
  R: Read,
{
  let mut bytes = [0u8; 8];
  r.read_exact(&mut bytes).map_err(ParseError::Io)?;
  Ok(u64::from_le_bytes(bytes))
}

#[cfg(test)]
mod tests {
  use std::io::Cursor;

  use crate::instruction::Instruction;

  use super::{Chunk, Constant};

  #[test]
  fn constant() {
    let constant = Constant::Int(42);
    let bytes = constant.to_bytes();
    assert_eq!(
      constant,
      Constant::from_bytes(&mut Cursor::new(bytes))
        .expect("failed to parse constant")
    );
  }

  #[test]
  fn chunk() {
    let mut chunk = Chunk::new();
    chunk.add_constant(Constant::Int(42));
    chunk.add_constant(Constant::Float(3.14159));
    chunk.add_instruction(Instruction::Add as u64);
    chunk.add_instruction(Instruction::Return as u64);
    let bytes = chunk.to_bytes();
    assert_eq!(
      chunk,
      Chunk::from_bytes(&mut Cursor::new(bytes))
        .expect("failed to parse constant")
    );
  }
}

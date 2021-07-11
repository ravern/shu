use std::{
  collections::HashMap,
  fmt,
  io::{self, Read},
};

use num_traits::FromPrimitive;

use crate::instruction::Instruction;

#[derive(Debug)]
pub enum ParseError {
  InvalidConstantType(u64),
  InvalidConstantValue(u64),
  InvalidInstruction(u64),
  Io(io::Error),
}

#[derive(Debug, PartialEq)]
pub struct Executable {
  chunks: Vec<Chunk>,
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
  locals: HashMap<String, usize>,
  constants: Vec<Constant>,
  code: Vec<u8>,
}

impl Chunk {
  pub fn new() -> Chunk {
    Chunk {
      locals: HashMap::new(),
      constants: Vec::new(),
      code: Vec::new(),
    }
  }

  pub fn add_constant(&mut self, constant: Constant) -> usize {
    self.constants.push(constant);
    self.constants.len() - 1
  }

  pub fn add_local(&mut self, identifier: String) -> usize {
    let index = self.locals.len();
    self.locals.insert(identifier, index);
    index
  }

  pub fn emit(&mut self, instruction: Instruction) -> usize {
    self.code.push(instruction as u8);
    self.code.len() - 1
  }

  pub fn emit_byte(&mut self, byte: u8) -> usize {
    self.code.push(byte);
    self.code.len() - 1
  }

  pub fn emit_bytes<const N: usize>(&mut self, bytes: [u8; N]) -> usize {
    for byte in bytes {
      self.code.push(byte);
    }
    self.code.len() - N
  }

  pub fn local(&self, identifier: &str) -> Option<usize> {
    self.locals.get(identifier).copied()
  }

  pub fn constant(&self, index: usize) -> Option<Constant> {
    self.constants.get(index).cloned()
  }

  pub fn code(&self) -> &[u8] {
    &self.code
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

    let code_len = read_u64(r)?;
    for _ in 0..code_len {
      let byte = read_u8(r)?;
      chunk.code.push(byte);
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

    let code_len = self.code.len() as u64;
    bytes.extend(code_len.to_le_bytes());
    bytes.extend(self.code.clone());

    bytes
  }
}

impl fmt::Display for Chunk {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut offset = 0;

    while offset < self.code.len() {
      write!(f, "{:#010x}", offset)?;

      offset += 1;
      let instruction = match Instruction::from_u8(self.code[offset - 1]) {
        Some(instruction) => {
          write!(f, " {:16}", format!("{}", instruction))?;
          instruction
        }
        None => {
          write!(f, " {:16}", "Invalid")?;
          continue;
        }
      };

      match instruction {
        Instruction::PushConstant => {
          if offset + 8 < self.code.len() {
            let index_bytes = [
              self.code[offset],
              self.code[offset + 1],
              self.code[offset + 2],
              self.code[offset + 3],
              self.code[offset + 4],
              self.code[offset + 5],
              self.code[offset + 6],
              self.code[offset + 7],
            ];
            let index = u64::from_le_bytes(index_bytes);
            write!(f, " {:#010x}", index)?;
            offset += 8;

            if let Some(constant) = self.constants.get(index as usize) {
              write!(f, " {}", constant)?;
            } else {
              write!(f, " Invalid")?;
            }
          }
        }
        Instruction::PushLocal => {
          if offset + 8 < self.code.len() {
            let index_bytes = [
              self.code[offset],
              self.code[offset + 1],
              self.code[offset + 2],
              self.code[offset + 3],
              self.code[offset + 4],
              self.code[offset + 5],
              self.code[offset + 6],
              self.code[offset + 7],
            ];
            let index = u64::from_le_bytes(index_bytes);
            write!(f, " {:#010x}", index)?;
            offset += 8;
          }
        }
        _ => {}
      }

      writeln!(f, "")?;
    }

    Ok(())
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
  Int(i64),
  Float(f64),
  Bool(bool),
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
      3 => {
        let mut bytes = [0u8; 1];
        r.read_exact(&mut bytes).map_err(ParseError::Io)?;
        match bytes[0] {
          0 => Constant::Bool(false),
          1 => Constant::Bool(true),
          byte => return Err(ParseError::InvalidConstantValue(byte as u64)),
        }
      }
      constant => {
        return Err(ParseError::InvalidConstantType(constant));
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
      Constant::Bool(bool) => {
        bytes.extend(3u64.to_le_bytes());
        if *bool {
          bytes.push(1);
        } else {
          bytes.push(0);
        }
      }
    }

    bytes
  }
}

impl fmt::Display for Constant {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Constant::Int(int) => write!(f, "{}", int),
      Constant::Float(float) => write!(f, "{}", float),
      Constant::Bool(bool) => write!(f, "{}", bool),
    }
  }
}

fn read_u8<R>(r: &mut R) -> Result<u8, ParseError>
where
  R: Read,
{
  let mut bytes = [0u8; 1];
  r.read_exact(&mut bytes).map_err(ParseError::Io)?;
  Ok(bytes[0])
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
    chunk.add_constant(Constant::Bool(false));
    chunk.emit(Instruction::Add);
    chunk.emit(Instruction::Return);
    let bytes = chunk.to_bytes();
    assert_eq!(
      chunk,
      Chunk::from_bytes(&mut Cursor::new(bytes))
        .expect("failed to parse constant")
    );
  }
}

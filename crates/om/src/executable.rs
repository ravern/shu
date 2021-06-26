use std::rc::Rc;

use crate::value::Atom;

pub struct Executable {
  chunks: Vec<Rc<Chunk>>,
  constants: Vec<Atom>,
}

impl Executable {
  pub fn chunk(&self, index: usize) -> Option<&Rc<Chunk>> {
    self.chunks.get(index)
  }

  pub fn constant(&self, index: usize) -> Option<&Atom> {
    self.constants.get(index)
  }
}

pub struct Chunk {
  code: Vec<u8>,
}

impl Chunk {
  pub fn op(&self, index: usize) -> Option<u8> {
    self.code.get(index).cloned()
  }
}

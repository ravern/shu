use crate::value::Atom;

pub struct Executable {
  code: Vec<u8>,
  constants: Vec<Atom>,
}

impl Executable {
  pub fn op(&self, index: usize) -> Option<u8> {
    self.code.get(index).cloned()
  }

  pub fn constant(&self, index: usize) -> Option<Atom> {
    self.constants.get(index).cloned()
  }
}

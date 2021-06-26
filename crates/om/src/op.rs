use std::convert::TryInto;

use std::convert::AsMut;

use crate::Error;

pub const OP_HALT: u8 = 0;
pub const OP_PUSH: u8 = 1;
pub const OP_SAVE: u8 = 2;
pub const OP_LOAD: u8 = 3;
pub const OP_JUMP: u8 = 4;
pub const OP_JUMP_IF: u8 = 5;
pub const OP_CALL: u8 = 6;
pub const OP_RETURN: u8 = 7;

const USIZE_SIZE: usize = std::mem::size_of::<usize>();

pub fn read_usize(bytes: &[u8]) -> Result<usize, Error> {
  if bytes.len() >= USIZE_SIZE {
    Ok(usize::from_le_bytes(copy_into_array(&bytes[0..USIZE_SIZE])))
  } else {
    Err(Error::SegmentationFault)
  }
}

// From https://stackoverflow.com/a/50080940
fn copy_into_array<A, T>(slice: &[T]) -> A
where
  A: Default + AsMut<[T]>,
  T: Copy,
{
  let mut a = A::default();
  <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
  a
}

use std::fmt;

use crate::executable::Constant;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
  Unit,
  Int(i64),
  Float(f64),
  Bool(bool),
}

impl From<Constant> for Value {
  fn from(constant: Constant) -> Self {
    match constant {
      Constant::Int(int) => Value::Int(int),
      Constant::Float(float) => Value::Float(float),
      Constant::Bool(bool) => Value::Bool(bool),
    }
  }
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Value::Unit => write!(f, "()"),
      Value::Int(int) => write!(f, "{}", int),
      Value::Float(float) => write!(f, "{}", float),
      Value::Bool(bool) => write!(f, "{}", bool),
    }
  }
}

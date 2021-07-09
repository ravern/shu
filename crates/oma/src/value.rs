use crate::executable::Constant;

#[derive(Debug, PartialEq)]
pub enum Value {
  Int(i64),
  Float(f64),
}

impl From<Constant> for Value {
  fn from(constant: Constant) -> Self {
    match constant {
      Constant::Int(int) => Value::Int(int),
      Constant::Float(float) => Value::Float(float),
    }
  }
}

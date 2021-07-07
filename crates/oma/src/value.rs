pub enum Value {
  Int(i64),
  Float(f64),
  Bool(bool),
  String(String),
  Closure(Closure),
  Struct(Struct),
  Enum(Enum),
}

pub struct Closure {}

pub struct Struct {}

pub struct Enum {}

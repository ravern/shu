pub struct Executable {
  symbols: Vec<String>,
  chunks: Vec<Chunk>,
}

pub struct Chunk {
  constants: Vec<Constant>,
  instructions: Vec<usize>,
}

pub enum Constant {
  Int(i64),
  Float(f64),
}

use std::fs;

use oma_bootstrap::{compile::Compiler, parse::Parser};

fn main() {
  let compiler = Compiler::new();

  // let generator = Generator::new();
  // let mut machine = Machine::new();

  let executable = compiler.compile();
  println!("{:#?}", executable);

  // let chunk = generator.generate(file);
  // println!("{}", chunk);

  // let result = machine.execute(&chunk).expect("execution error");
  // println!("{}", result);
}

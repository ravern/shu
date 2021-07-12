use std::fs;

use oma::machine::Machine;
use oma_bootstrap::{gen::Generator, parse::Parser};

fn main() {
  let source =
    fs::read_to_string("examples/test.oma").expect("couldn't read file");

  let parser = Parser::new(&source);
  let generator = Generator::new();
  let mut machine = Machine::new();

  let chunk = generator.generate(parser.parse());
  println!("{}", chunk);

  let result = machine.execute(&chunk).expect("execution error");
  println!("{}", result);
}

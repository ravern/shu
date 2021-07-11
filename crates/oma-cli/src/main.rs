use oma::machine::Machine;
use oma_bootstrap::{gen::Generator, parse::Parser};

fn main() {
  let mut editor = rustyline::Editor::<()>::new();
  let mut machine = Machine::new();

  loop {
    let line = editor.readline("> ").unwrap();

    let parser = Parser::new(&line);
    let expression = parser.parse();

    let generator = Generator::new();
    let chunk = generator.generate(expression);

    println!("{}", chunk);
    println!("{:?}", machine.execute(&chunk));
  }
}

use oma::Machine;
use oma_bootstrap::Compiler;

enum Error {
  Compile(om_bootstrap::CompileError),
  Execution(oma::Error),
}

const SOURCE: &'static str = include_str!("../src/web_server.oma");

fn main() -> Result<(), Error> {
  let compiler = Compiler::new();
  let executable = compiler.compile(SOURCE).map_err(Error::Compile)?;

  let machine = Machine::new();
  machine.execute(executable).map_err(Error::Execution)?;

  Ok(())
}

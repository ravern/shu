use crate::{context::Context, error::Error};

struct Machine {
  context: Context,
}

impl Machine {
  fn new() -> Machine {
    Machine {
      context: Context::new(),
    }
  }

  fn run_file(&mut self, path: &str) -> Result<(), Error> {
    Ok(())
  }
}

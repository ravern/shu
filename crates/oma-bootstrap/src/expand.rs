use thiserror::Error;

use crate::ast::*;

#[derive(Debug, Error)]
pub enum ExpandError {}

fn expand(exprs: Vec<Expr>) -> Result<Vec<Expr>, ExpandError> {
  Ok(Expr::List(List::Nil))
}

/// Expands macros fully in preparation for compilation.
struct Expander {}

impl Expander {
  fn new() -> Expander {
    Expander {}
  }

  fn expand(&mut self, expr: Expr) -> Result<Expr, ExpandError> {}
}

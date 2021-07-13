use std::{collections::HashMap, fs, io};

use crate::{
  ast,
  ir::{Block, Chunk, Executable, ModHeader, PackageHeader, Path},
  parse::{ParseError, Parser},
  span::Spanned,
};

#[derive(Debug)]
pub enum CompileError {
  Io(io::Error),
  FileNotFound,
  Parse(Vec<Spanned<ParseError>>),
}

pub struct Compiler {
  identifiers: HashMap<String, usize>,
  chunks: Vec<Chunk>,
}

impl Compiler {
  pub fn new() -> Compiler {
    Compiler {
      identifiers: HashMap::new(),
      chunks: Vec::new(),
    }
  }

  pub fn compile(mut self) -> Result<Executable, CompileError> {
    let source =
      fs::read_to_string("examples/src/lib.oma").map_err(CompileError::Io)?;
    let parser = Parser::new(&source);
    let file = parser.parse().map_err(CompileError::Parse)?;

    let package_header = self.package_header(file)?;

    Ok(Executable {
      package_header,
      chunks: self.chunks,
      identifiers: self.identifiers,
    })
  }

  fn package_header(
    &mut self,
    file: ast::File,
  ) -> Result<PackageHeader, CompileError> {
    let mut use_declarations = Vec::new();
    let mut mod_headers = HashMap::new();
    let mut fn_headers = HashMap::new();

    for declaration in file.declarations {
      match declaration {
        ast::Declaration::Use(use_declaration) => {
          use_declarations.extend(self.use_declaration(use_declaration)?);
        }
        ast::Declaration::Mod(mod_declaration) => {
          let name =
            self.add_identifier(mod_declaration.name.span().to_string());
          mod_headers.insert(name, self.mod_header(mod_declaration)?);
        }
        ast::Declaration::Fn(fn_declaration) => {
          let name =
            self.add_identifier(fn_declaration.name.span().to_string());
          fn_headers.insert(name, self.fn_header(fn_declaration)?);
        }
      };
    }

    Ok(PackageHeader {
      package_headers: HashMap::new(),
      use_declarations,
      mod_headers,
      fn_headers,
    })
  }

  fn mod_header(
    &mut self,
    mod_declaration: ast::ModDeclaration,
  ) -> Result<ModHeader, CompileError> {
    let body = if let Some(body) = mod_declaration.body {
      body
    } else {
      let source = fs::read_to_string(format!(
        "examples/src/{}.oma",
        mod_declaration.name.span().as_str()
      ))
      .map_err(CompileError::Io)?;
      let parser = Parser::new(&source);
      parser.parse().map_err(CompileError::Parse)?
    };

    let mut use_declarations = Vec::new();
    let mut mod_headers = HashMap::new();
    let mut fn_headers = HashMap::new();

    for declaration in body.declarations {
      match declaration {
        ast::Declaration::Use(use_declaration) => {
          use_declarations.extend(self.use_declaration(use_declaration)?);
        }
        ast::Declaration::Mod(mod_declaration) => {
          let name =
            self.add_identifier(mod_declaration.name.span().to_string());
          mod_headers.insert(name, self.mod_header(mod_declaration)?);
        }
        ast::Declaration::Fn(fn_declaration) => {
          let name =
            self.add_identifier(fn_declaration.name.span().to_string());
          fn_headers.insert(name, self.fn_header(fn_declaration)?);
        }
      };
    }

    Ok(ModHeader {
      use_declarations,
      mod_headers,
      fn_headers,
    })
  }

  fn fn_header(
    &mut self,
    fn_declaration: ast::FnDeclaration,
  ) -> Result<usize, CompileError> {
    let parameters = fn_declaration
      .parameters
      .iter()
      .map(|token| self.add_identifier(token.span().to_string()))
      .collect();

    let chunk = self.add_chunk(Chunk {
      parameters,
      body: Block {
        statements: Vec::new(),
      },
    });

    Ok(chunk)
  }

  fn use_declaration(
    &mut self,
    use_declaration: ast::UseDeclaration,
  ) -> Result<Vec<Path>, CompileError> {
    Ok(
      use_declaration
        .trees
        .into_iter()
        .map(|use_tree| self.use_tree(use_tree))
        .collect::<Result<Vec<Vec<Vec<usize>>>, CompileError>>()?
        .into_iter()
        .flatten()
        .map(|components| Path { components })
        .collect(),
    )
  }

  fn use_tree(
    &mut self,
    use_tree: ast::UseTree,
  ) -> Result<Vec<Vec<usize>>, CompileError> {
    match use_tree {
      ast::UseTree::Branch(branch) => {
        let component =
          self.add_identifier(branch.component.span().to_string());

        Ok(
          branch
            .subtrees
            .into_iter()
            .map(|use_tree| self.use_tree(use_tree))
            .collect::<Result<Vec<Vec<Vec<usize>>>, CompileError>>()?
            .into_iter()
            .flatten()
            .map(|components| {
              let mut components = components.clone();
              components.push(component);
              components
            })
            .collect(),
        )
      }
      ast::UseTree::Leaf(token) => {
        Ok(vec![vec![self.add_identifier(token.span().to_string())]])
      }
    }
  }

  fn add_chunk(&mut self, chunk: Chunk) -> usize {
    self.chunks.push(chunk);
    self.chunks.len() - 1
  }

  fn add_identifier(&mut self, identifier: String) -> usize {
    if let Some(identifier) = self.identifiers.get(&identifier) {
      *identifier
    } else {
      self.identifiers.insert(identifier, self.identifiers.len());
      self.identifiers.len() - 1
    }
  }
}

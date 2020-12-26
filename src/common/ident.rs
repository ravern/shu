use internment::Intern;

#[derive(Debug, PartialEq)]
pub struct Ident(Intern<String>);

impl Ident {
  pub fn new(ident: String) -> Ident {
    Ident(Intern::new(ident))
  }
}

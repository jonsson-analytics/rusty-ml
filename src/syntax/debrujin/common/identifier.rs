#[derive(Debug, Clone, PartialEq)]
pub struct Identifier
{
  pub name: usize,
}

impl Identifier
{
  pub fn new(name: usize) -> Self
  {
    Self {
      name,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier
{
  pub name: String,
}

impl Identifier
{
  pub fn new<IntoString>(name: IntoString) -> Self
  where
    IntoString: Into<String>,
  {
    Self {
      name: name.into(),
    }
  }
}

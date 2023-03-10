#[derive(Debug, Clone, PartialEq)]
pub enum Literal
{
  String(String),
  Number(f64),
  Boolean(bool),
}

impl Literal
{
  pub fn string<IntoString>(value: IntoString) -> Self
  where
    IntoString: Into<String>,
  {
    Self::String(value.into())
  }
}

impl From<&str> for Literal
{
  fn from(value: &str) -> Self
  {
    Self::String(value.into())
  }
}


impl From<String> for Literal
{
  fn from(value: String) -> Self
  {
    Self::String(value)
  }
}

impl From<bool> for Literal
{
  fn from(value: bool) -> Self
  {
    Self::Boolean(value)
  }
}

impl From<f64> for Literal
{
  fn from(value: f64) -> Self
  {
    Self::Number(value)
  }
}

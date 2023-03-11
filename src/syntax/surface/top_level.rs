use super::Val;

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel
{
  Val(Box<Val>),
}

impl From<Val> for TopLevel
{
  fn from(val: Val) -> Self
  {
    Self::Val(Box::new(val))
  }
}

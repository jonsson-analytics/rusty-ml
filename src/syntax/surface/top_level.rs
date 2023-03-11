use super::{
  ValBinding,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel
{
  ValBinding(Box<ValBinding>),
}

impl From<ValBinding> for TopLevel
{
  fn from(val: ValBinding) -> Self
  {
    Self::ValBinding(Box::new(val))
  }
}

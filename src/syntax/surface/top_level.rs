use super::{
  DefBinding,
  ValBinding,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel
{
  DefBinding(Box<DefBinding>),
  ValBinding(Box<ValBinding>),
}

impl From<DefBinding> for TopLevel
{
  fn from(def: DefBinding) -> Self
  {
    Self::DefBinding(Box::new(def))
  }
}

impl From<ValBinding> for TopLevel
{
  fn from(val: ValBinding) -> Self
  {
    Self::ValBinding(Box::new(val))
  }
}

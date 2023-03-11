pub use crate::syntax::surface::ValBinding;

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel
{
  Val(Box<ValBinding>),
}

impl From<ValBinding> for TopLevel
{
  fn from(val: ValBinding) -> Self
  {
    Self::Val(Box::new(val))
  }
}

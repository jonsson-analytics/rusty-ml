use crate::syntax::surface::{
  Expression,
  Identifier,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Val
{
  pub name: Identifier,
  pub value: Expression,
}

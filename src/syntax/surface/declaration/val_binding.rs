use crate::syntax::surface::{
  Expression,
  Identifier,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ValBinding
{
  pub name: Identifier,
  pub value: Expression,
}

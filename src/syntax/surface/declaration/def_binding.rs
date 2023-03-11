use crate::syntax::surface::{
  Expression,
  Identifier,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DefBinding
{
  pub name: Identifier,
  pub value: Expression,
}

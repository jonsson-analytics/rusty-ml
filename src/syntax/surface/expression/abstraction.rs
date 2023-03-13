use super::{
  Expression,
  Identifier,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Abstraction
{
  pub parameters: Vec<Identifier>,
  pub body: Expression,
}

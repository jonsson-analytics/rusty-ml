use super::{Identifier, Expression};

#[derive(Debug, Clone, PartialEq)]
pub struct Abstraction
{
  pub parameters: Vec<Identifier>,
  pub body: Expression,
}

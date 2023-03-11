use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Abstraction
{
  pub body: Expression,
}

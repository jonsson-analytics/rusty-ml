use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Application
{
  pub abstraction: Expression,
  pub argument: Expression,
}

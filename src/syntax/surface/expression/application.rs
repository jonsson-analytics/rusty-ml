use super::Expression;


#[derive(Debug, Clone, PartialEq)]
pub struct Application
{
  pub abstraction: Expression,
  pub arguments: Vec<Expression>,
}

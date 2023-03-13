use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Variable
{
  Named(Identifier),
  Unnamed(usize),
}

impl From<Identifier> for Variable
{
  fn from(identifier: Identifier) -> Self
  {
    Self::Named(identifier)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Abstraction
{
  pub parameter_type: Type,
  pub return_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type
{
  Variable(Variable),
  Concrete(Identifier),
  Abstraction(Box<Abstraction>)
}

impl Type
{
  pub fn abstraction(
    parameter_type: Type,
    return_type: Type,
  ) -> Type
  {
    types::Abstraction {
      parameter_type,
      return_type,
    }
    .into()
  }
}

impl From<Identifier> for Type
{
  fn from(identifier: Identifier) -> Self
  {
    Self::Concrete(identifier)
  }
}

impl From<Variable> for Type
{
  fn from(variable: Variable) -> Self
  {
    Self::Variable(variable)
  }
}

impl From<Abstraction> for Type
{
  fn from(abstraction: Abstraction) -> Self
  {
    Self::Abstraction(Box::new(abstraction))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint
{
  Equivalent(Equivalent),
}

impl From<Equivalent> for Constraint
{
  fn from(equivalent: Equivalent) -> Self
  {
    Constraint::Equivalent(equivalent)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Equivalent
{
  pub left: Type,
  pub right: Type,
}

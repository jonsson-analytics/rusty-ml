use super::*;

#[derive(Debug, Clone, PartialEq)]
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
pub struct Polymorphic
{
  pub name: Variable,
  pub parameters: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type
{
  Polymorphic(Polymorphic),
  Variable(Variable),
}

impl Type
{
  pub fn monomorphic(name: Variable) -> Type
  {
    types::Polymorphic {
      name,
      parameters: vec![],
    }
    .into()
  }

  pub fn abstraction(
    parameter_type: Type,
    return_type: Type,
  ) -> Type
  {
    types::Polymorphic {
      name: Identifier::new("->").into(),
      parameters: vec![parameter_type, return_type],
    }
    .into()
  }
}

impl From<Variable> for Type
{
  fn from(variable: Variable) -> Self
  {
    Self::Variable(variable)
  }
}

impl From<Polymorphic> for Type
{
  fn from(polymorphic: Polymorphic) -> Self
  {
    Self::Polymorphic(polymorphic)
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

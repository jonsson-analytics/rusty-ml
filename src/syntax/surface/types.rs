use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Variable
{
  Named(Identifier),
  Unnamed(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type
{
  Polymorphic
  {
    name: Identifier,
    parameters: Vec<Identifier>,
  },
  Variable(Variable),
}
